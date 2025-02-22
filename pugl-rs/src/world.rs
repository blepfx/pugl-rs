use crate::{Backend, UnrealizedView, sys};
use std::{ffi::CStr, os::raw::c_void, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldError;
impl std::error::Error for WorldError {}
impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown pugl world error")
    }
}

/// The "world" of application state.
///
/// The world represents everything that is not associated with a particular view.
/// Several worlds can be created in a single process, but code using different worlds must be isolated so they are never mixed.
/// Views are strongly associated with the world they were created in.
#[repr(transparent)]
pub struct World {
    world: *mut sys::PuglWorld,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

///A pointer to the native handle of the world.
/// - X11: Returns a pointer to the Display.
/// - MacOS: Returns a pointer to the NSApplication.
/// - Windows: Returns the HMODULE of the calling process.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeWorld {
    ptr: *mut c_void,
}

impl NativeWorld {
    pub fn as_raw(&self) -> *mut c_void {
        self.ptr
    }

    pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NativeWorld {}
unsafe impl Sync for NativeWorld {}

impl World {
    pub(crate) fn from_inner(world: &*mut sys::PuglWorld) -> &Self {
        unsafe { &*(world as *const *mut sys::PuglWorld as *const Self) }
    }

    /// Create a new world in a `PROGRAM` mode. Used for top-level applications.
    pub fn new_program() -> Result<Self, WorldError> {
        unsafe {
            let world = sys::puglNewWorld(sys::PUGL_PROGRAM, 0);
            if world.is_null() {
                Err(WorldError)
            } else {
                Ok(Self { world })
            }
        }
    }

    /// Create a new world in a `MODULE` mode. Used for plugins or modules within a larger applications.
    pub fn new_module() -> Result<Self, WorldError> {
        unsafe {
            let world = sys::puglNewWorld(sys::PUGL_MODULE, 0);
            if world.is_null() {
                Err(WorldError)
            } else {
                Ok(Self { world })
            }
        }
    }

    /// Sets the application class name.
    ///
    /// This is a stable identifier for the application, which should be a short camel-case name like "MyApp". This should be the same for every instance of the application, but different from any other application. On X11 and Windows, it is used to set the class name of windows (that underlie realized views), which is used for things like loading configuration, or custom window management rules.
    pub fn with_class_name(self, string: &str) -> Self {
        unsafe {
            sys::puglSetWorldString(
                self.world,
                sys::PUGL_CLASS_NAME,
                string.as_ptr() as *const _,
            );
        }
        self
    }

    /// Gets the class name of the application.
    /// See `with_class_name` for more information.
    pub fn class_name(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::puglGetWorldString(self.world, sys::PUGL_CLASS_NAME))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Return the time in seconds
    /// This is a monotonically increasing clock with high resolution. The returned time is only useful to compare against other times returned by this function, its absolute value has no meaning.
    pub fn time(&self) -> f64 {
        unsafe { sys::puglGetTime(self.world) }
    }

    /// Update by processing events from the window system.
    /// - This function is a single iteration of the main loop, and should be called repeatedly to update all views.
    /// - If `timeout` is `None`, this function will block until an event is received. If `timeout` is `Some(duration)`, this function will block for at most `duration` before returning.
    /// - For continuously animating programs, a timeout that is a reasonable fraction of the ideal frame period should be used, to minimize input latency by ensuring that as many input events are consumed as possible before drawing.
    /// - Returns `true` if an event was received, `false` if the timeout was reached
    pub fn update(&mut self, timeout: Option<Duration>) -> Result<bool, WorldError> {
        unsafe {
            let timeout = timeout.map(|d| d.as_secs_f64()).unwrap_or(-1.0);
            match sys::puglUpdate(self.world, timeout) {
                sys::PUGL_SUCCESS => Ok(true),
                sys::PUGL_FAILURE => Ok(false),
                _ => Err(WorldError),
            }
        }
    }

    /// Return a pointer to the native handle of the world.
    /// - X11: Returns a pointer to the Display.
    /// - MacOS: Returns a pointer to the NSApplication.
    /// - Windows: Returns the HMODULE of the calling process.
    pub fn as_native(&self) -> NativeWorld {
        unsafe {
            NativeWorld {
                ptr: sys::puglGetNativeWorld(self.world) as *mut c_void,
            }
        }
    }

    /// Creates a new unrealized view with a specified backend.
    /// Available backends are:
    /// - `()` - stub backend, no drawing
    pub fn new_view<B: Backend>(&self, backend: B) -> UnrealizedView<B> {
        unsafe { UnrealizedView::new(self.world, backend) }
    }
}

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            sys::puglFreeWorld(self.world);
        }
    }
}
