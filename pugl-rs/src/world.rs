use crate::{Backend, UnrealizedView, sys};
use std::{
    any::Any,
    ffi::CStr,
    mem::{ManuallyDrop, replace},
    os::raw::c_void,
    panic::resume_unwind,
    sync::{Arc, Mutex},
    time::Duration,
};

/// World creation/update error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldError;

impl std::error::Error for WorldError {}
impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown pugl world error")
    }
}

/// The entry point of a Pugl application.
///
/// The world represents everything that is not associated with a particular view.
/// Several worlds can be created in a single process,
/// but code using different worlds must be isolated so they are never mixed.
/// Views are strongly associated with the world they were created in.
#[repr(transparent)]
pub struct World(Arc<WorldInner>);

unsafe impl Send for World {}
unsafe impl Sync for World {}

///A pointer to the native handle of the world.
/// - X11: A pointer to the `Display`.
/// - MacOS: A pointer to the `NSApplication`.
/// - Windows: The `HMODULE` of the calling process.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeWorld {
    ptr: *mut c_void,
}

impl NativeWorld {
    /// Returns a raw pointer to the native handle of the world.
    pub fn as_raw(&self) -> *mut c_void {
        self.ptr
    }

    /// Constructs a `NativeWorld` from a raw pointer.
    pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NativeWorld {}
unsafe impl Sync for NativeWorld {}

impl World {
    /// Create a new world in a `PROGRAM` mode.
    ///
    /// Used for top-level applications.
    pub fn new_program() -> Result<Self, WorldError> {
        unsafe {
            let world = sys::puglNewWorld(sys::PUGL_PROGRAM, 0);
            if world.is_null() {
                Err(WorldError)
            } else {
                Ok(Self(WorldInner::wrap(world)))
            }
        }
    }

    /// Create a new world in a `MODULE` mode.
    ///
    /// Used for plugins or modules within a larger applications.
    pub fn new_module() -> Result<Self, WorldError> {
        unsafe {
            let world = sys::puglNewWorld(sys::PUGL_MODULE, sys::PUGL_WORLD_THREADS);
            if world.is_null() {
                Err(WorldError)
            } else {
                Ok(Self(WorldInner::wrap(world)))
            }
        }
    }

    /// Sets the application class name.
    ///
    /// This is a stable identifier for the application, which should be a short camel-case name like "MyApp". This should be the same for every instance of the application, but different from any other application.
    /// On X11 and Windows, it is used to set the class name of windows (that underlie realized views), which is used for things like loading configuration, or custom window management rules.
    pub fn with_class_name(self, string: &str) -> Self {
        unsafe {
            sys::puglSetWorldString(
                self.0.raw,
                sys::PUGL_CLASS_NAME,
                string.as_ptr() as *const _,
            );
        }
        self
    }

    /// Gets the class name of the application.
    ///
    /// See [`World::with_class_name`] for more information.
    pub fn class_name(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::puglGetWorldString(self.0.raw, sys::PUGL_CLASS_NAME))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Return the time in seconds
    ///
    /// This is a monotonically increasing clock with high resolution. The returned time is only useful to compare against other times returned by this function, its absolute value has no meaning.
    pub fn time(&self) -> f64 {
        unsafe { sys::puglGetTime(self.0.raw) }
    }

    /// Update by processing events from the window system.
    /// - This function is a single iteration of the main loop, and should be called repeatedly to update all views.
    /// - If `timeout` is `None`, this function will block until an event is received. If `timeout` is `Some(duration)`, this function will block for at most `duration` before returning.
    /// - For continuously animating programs, a timeout that is a reasonable fraction of the ideal frame period should be used, to minimize input latency by ensuring that as many input events are consumed as possible before drawing.
    /// - Returns `true` if an event was received, `false` if the timeout was reached
    pub fn update(&mut self, timeout: Option<Duration>) -> Result<bool, WorldError> {
        if let Some(poison) = self.0.replace_poison(None) {
            resume_unwind(poison);
        }

        unsafe {
            let timeout = timeout.map(|d| d.as_secs_f64()).unwrap_or(-1.0);
            match sys::puglUpdate(self.0.raw, timeout) {
                sys::PUGL_SUCCESS => Ok(true),
                sys::PUGL_FAILURE => Ok(false),
                _ => Err(WorldError),
            }
        }
    }

    /// Return a pointer to the native handle of the world.
    ///
    /// See [`NativeWorld`] for more info.
    pub fn native(&self) -> NativeWorld {
        unsafe {
            NativeWorld {
                ptr: sys::puglGetNativeWorld(self.0.raw) as *mut c_void,
            }
        }
    }

    /// Creates a new unrealized view with a specified backend.
    ///
    /// See [`Backend`] for more info.
    pub fn new_view<B: Backend>(&self, backend: B) -> UnrealizedView<B> {
        unsafe { UnrealizedView::new(self.0.clone(), backend) }
    }
}

pub(crate) struct WorldInner {
    pub raw: *mut sys::PuglWorld,
    pub poison: Mutex<Option<Box<dyn Any + Send>>>,
}

impl WorldInner {
    pub fn wrap(world: *mut sys::PuglWorld) -> Arc<Self> {
        unsafe {
            let arc = Arc::new(WorldInner {
                raw: world,
                poison: Mutex::new(None),
            });

            sys::puglSetWorldHandle(world, Arc::as_ptr(&arc) as _);
            arc
        }
    }

    /// SAFETY: do not drop this arc after you're done with it!
    pub unsafe fn from_raw(world: *mut sys::PuglWorld) -> ManuallyDrop<Arc<Self>> {
        unsafe { ManuallyDrop::new(Arc::from_raw(sys::puglGetWorldHandle(world) as *const Self)) }
    }

    pub fn as_world(&self) -> &World {
        unsafe { &*(self as *const _ as *const World) }
    }

    pub fn replace_poison(
        &self,
        panic: Option<Box<dyn Any + Send>>,
    ) -> Option<Box<dyn Any + Send>> {
        self.poison.clear_poison();
        replace(&mut self.poison.lock().unwrap(), panic)
    }
}

impl Drop for WorldInner {
    fn drop(&mut self) {
        unsafe {
            sys::puglFreeWorld(self.raw);
        }
    }
}
