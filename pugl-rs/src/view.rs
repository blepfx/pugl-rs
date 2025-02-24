use crate::{
    Backend, Event, MouseCursor, Rect, TimerId, ViewStyle, ViewType, World, WorldInner, sys,
};
use std::{
    ffi::CString,
    fmt,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::null_mut,
    sync::{Arc, Mutex},
    time::Duration,
};

/// A view that is not yet realized.
///
/// This struct represents a view that is yet to be "realized" (i.e. created on the underlying OS windowing system).
pub struct UnrealizedView<B: Backend>(View<B>);

/// A drawable area that can receive input events.
///
/// This struct represents a view that has been "realized" (i.e. created on the underlying OS windowing system).
pub struct View<B: Backend> {
    pub(crate) view: *mut sys::PuglView,
    pub(crate) world: Arc<WorldInner>,
    pub(crate) phantom: PhantomData<B>,
}

/// Represents a parent window for a view.
///
/// A view can either have a parent (for embedding) or a transient parent (for top-level windows like dialogs), but not both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewParent {
    Embedding(NativeView),
    Transient(NativeView),
}

/// A native view handle.
/// - X11: This is a `Window`.
/// - MacOS: This is a pointer to an `NSView`.
/// - Windows: This is a `HWND`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeView {
    ptr: sys::PuglNativeView,
}

impl NativeView {
    /// Returns the underlying OS window handle
    pub fn as_raw(&self) -> usize {
        self.ptr
    }

    /// Constructs a `NativeView` from a raw window handle.
    /// It is assumed that the handle is valid.
    pub unsafe fn from_raw(ptr: usize) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NativeView {}
unsafe impl Sync for NativeView {}

// TODO: verify that these are correct
// pugl docs have no thread safety info
unsafe impl<B: Backend> Send for View<B> {}
unsafe impl<B: Backend> Sync for View<B> {}
unsafe impl<B: Backend> Send for UnrealizedView<B> {}
unsafe impl<B: Backend> Sync for UnrealizedView<B> {}

impl<B: Backend> UnrealizedView<B> {
    pub(crate) unsafe fn new(world: Arc<WorldInner>, backend: B) -> Self {
        unsafe {
            let view = sys::puglNewView(world.raw);
            assert!(!view.is_null(), "failed to allocate view");
            sys::puglSetEventFunc(view, Some(event_handler::<B>));
            sys::puglSetHandle(view, null_mut());
            backend.install(view, crate::private::Private);
            Self(View {
                view,
                world,
                phantom: PhantomData,
            })
        }
    }

    /// Set the type of the view. See the docs for [`ViewType`] for more info.
    pub fn with_view_type(self, ty: ViewType) -> Self {
        unsafe {
            sys::puglSetViewHint(self.0.view, sys::PUGL_VIEW_TYPE, ty.into_raw() as _);
        }
        self
    }

    /// Ignore key repeats when a key is held down.
    pub fn with_ignore_key_repeats(self, ignore: bool) -> Self {
        unsafe {
            sys::puglSetViewHint(
                self.0.view,
                sys::PUGL_IGNORE_KEY_REPEAT,
                if ignore { 1 } else { 0 },
            );
        }
        self
    }

    /// Make the window resizable
    pub fn with_resizable(self, resizable: bool) -> Self {
        unsafe {
            sys::puglSetViewHint(
                self.0.view,
                sys::PUGL_RESIZABLE,
                if resizable { 1 } else { 0 },
            );
        }
        self
    }

    /// Set the update rate in Hz
    pub fn with_refresh_rate(self, rate: u32) -> Self {
        unsafe {
            sys::puglSetViewHint(self.0.view, sys::PUGL_REFRESH_RATE, rate as _);
        }
        self
    }

    /// Set the dark frame hint.
    pub fn with_dark_frame(self, dark: bool) -> Self {
        unsafe {
            sys::puglSetViewHint(self.0.view, sys::PUGL_DARK_FRAME, if dark { 1 } else { 0 });
        }
        self
    }

    /// Set the parent window of the view.
    pub fn with_parent(self, parent: ViewParent) -> Self {
        unsafe {
            match parent {
                ViewParent::Embedding(parent) => {
                    sys::puglSetParent(self.0.view, parent.ptr);
                }
                ViewParent::Transient(parent) => {
                    sys::puglSetTransientParent(self.0.view, parent.ptr);
                }
            }
        }
        self
    }

    /// Set the title of the window.
    pub fn with_title(self, title: &str) -> Self {
        self.0.set_title(title);
        self
    }

    /// Set the initial size of the view in (physical) pixels.
    pub fn with_size(self, width: u32, height: u32) -> Self {
        unsafe {
            sys::puglSetSizeHint(self.0.view, sys::PUGL_DEFAULT_SIZE, width, height);
        }
        self
    }

    /// Set the initial position of the view in screen coordinates with an upper left origin.
    pub fn with_position(self, x: i32, y: i32) -> Self {
        unsafe {
            sys::puglSetPositionHint(self.0.view, sys::PUGL_DEFAULT_POSITION, x, y);
        }
        self
    }

    /// Set the maximum size of the view in (physical) pixels.
    pub fn with_max_size(self, width: u32, height: u32) -> Self {
        self.0.set_max_size(width, height);
        self
    }

    /// Set the minimum size of the view in (physical) pixels.
    pub fn with_min_size(self, width: u32, height: u32) -> Self {
        self.0.set_min_size(width, height);
        self
    }

    /// Set the maximum aspect ratio of the view.
    pub fn with_max_aspect(self, x: u32, y: u32) -> Self {
        self.0.set_max_aspect(x, y);
        self
    }

    /// Set the minimum aspect ratio of the view.
    pub fn with_min_aspect(self, x: u32, y: u32) -> Self {
        self.0.set_min_aspect(x, y);
        self
    }

    /// Set the main event handler for the view.
    pub fn with_event_handler<E: FnMut(&View<B>, Event<B>) + Send + 'static>(
        self,
        event: E,
    ) -> Self {
        unsafe {
            let old = sys::puglGetHandle(self.0.view);
            if !old.is_null() {
                drop(Box::from_raw(old as *mut EventHandler<B>));
            }

            let event: Box<EventHandler<B>> = Box::new(Mutex::new(Box::new(event)));
            sys::puglSetHandle(self.0.view, Box::into_raw(event) as *mut _);
        }
        self
    }

    /// Returns the associated world instance
    pub fn world(&self) -> &World {
        self.0.world()
    }

    /// Return the parent window this view, if any
    pub fn parent(&self) -> Option<ViewParent> {
        self.0.parent()
    }

    /// Return the title of the window
    pub fn title(&self) -> String {
        self.0.title()
    }

    /// Return the scale factor of the view.
    ///
    /// This factor describe how large UI elements (especially text) should be compared to "normal". For example, 2.0 means the UI should be drawn twice as large.
    /// "Normal" is loosely defined, but means a good size on a "standard DPI" display (around 96 DPI).
    /// In other words, the scale 1.0 should have text that is reasonably sized on a 96 DPI display, and the scale 2.0 should have text twice that large.
    pub fn system_scale(&self) -> f64 {
        self.0.system_scale()
    }

    /// Realize the view
    ///
    /// Realize a view by creating a corresponding system view or window.
    /// After this call, the (initially invisible) underlying system view exists and can be accessed with [`View::native`].
    /// The view should be fully configured using the above functions before this is called. This function may only be called once per view.
    ///
    /// The view will be kept alive as long as the [`View`] instance is not dropped
    pub fn realize(self) -> Result<View<B>, ViewError> {
        unsafe {
            let error = match sys::puglRealize(self.0.view) {
                sys::PUGL_SUCCESS => return Ok(self.0),
                sys::PUGL_BAD_CONFIGURATION => ViewError::BadConfig,
                sys::PUGL_BAD_BACKEND => ViewError::BadBackend,
                sys::PUGL_BACKEND_FAILED => ViewError::BackendInit,
                sys::PUGL_REGISTRATION_FAILED => ViewError::ClassRegister,
                sys::PUGL_REALIZE_FAILED => ViewError::OsRealize,
                sys::PUGL_CREATE_CONTEXT_FAILED => ViewError::CreateContext,
                sys::PUGL_SET_FORMAT_FAILED => ViewError::SetPixelFormat,
                sys::PUGL_NO_MEMORY => ViewError::OutOfMemory,
                _ => ViewError::Unknown,
            };

            Err(error)
        }
    }
}

impl<B: Backend> View<B> {
    /// Set the maximum size of the view in (physical) pixels.
    pub fn set_max_size(&self, width: u32, height: u32) -> bool {
        unsafe {
            sys::puglSetSizeHint(self.view, sys::PUGL_MAX_SIZE, width, height) == sys::PUGL_SUCCESS
        }
    }

    /// Set the minimum size of the view in (physical) pixels.
    pub fn set_min_size(&self, width: u32, height: u32) -> bool {
        unsafe {
            sys::puglSetSizeHint(self.view, sys::PUGL_MIN_SIZE, width, height) == sys::PUGL_SUCCESS
        }
    }

    /// Set the maximum aspect ratio of the view.
    pub fn set_max_aspect(&self, x: u32, y: u32) -> bool {
        unsafe { sys::puglSetSizeHint(self.view, sys::PUGL_MAX_ASPECT, x, y) == sys::PUGL_SUCCESS }
    }

    /// Set the minimum aspect ratio of the view.
    pub fn set_min_aspect(&self, x: u32, y: u32) -> bool {
        unsafe { sys::puglSetSizeHint(self.view, sys::PUGL_MIN_ASPECT, x, y) == sys::PUGL_SUCCESS }
    }

    /// Set the current size of the view in (physical) pixels.
    pub fn set_size(&self, width: u32, height: u32) -> bool {
        unsafe {
            // workaround for not being able to resize the view when it's not marked as resizable
            if sys::puglGetViewHint(self.view, sys::PUGL_RESIZABLE) == 0 {
                sys::puglSetViewHint(self.view, sys::PUGL_RESIZABLE, 1);
                sys::puglSetSizeHint(self.view, sys::PUGL_MAX_SIZE, width, height);
                sys::puglSetSizeHint(self.view, sys::PUGL_MIN_SIZE, width, height);
                let result = sys::puglSetSizeHint(self.view, sys::PUGL_CURRENT_SIZE, width, height)
                    == sys::PUGL_SUCCESS;
                sys::puglSetViewHint(self.view, sys::PUGL_RESIZABLE, 0);
                result
            } else {
                sys::puglSetSizeHint(self.view, sys::PUGL_CURRENT_SIZE, width, height)
                    == sys::PUGL_SUCCESS
            }
        }
    }

    /// Set the current position of the view in screen coordinates with an upper left origin.
    pub fn set_position(&self, x: i32, y: i32) -> bool {
        unsafe {
            sys::puglSetPositionHint(self.view, sys::PUGL_CURRENT_POSITION, x, y)
                == sys::PUGL_SUCCESS
        }
    }

    /// Set the title of the window.
    pub fn set_title(&self, title: &str) -> bool {
        unsafe {
            sys::puglSetViewString(
                self.view,
                sys::PUGL_WINDOW_TITLE,
                CString::new(title).unwrap().as_ptr(),
            ) == sys::PUGL_SUCCESS
        }
    }

    /// Set the mouse cursor.
    ///
    /// This changes the system cursor that is displayed when the pointer is inside the view.
    ///
    /// The cursor is reset to the default when the pointer leaves the view.
    /// The cursor is also reset to the default when the view is hidden or obscured, so this function should be called in the event handler if the cursor should be changed back when the view is exposed again.
    pub fn set_cursor(&self, cursor: MouseCursor) -> bool {
        unsafe { sys::puglSetCursor(self.view, cursor.into_raw()) == sys::PUGL_SUCCESS }
    }

    /// Set a view state, if supported by the system.
    ///
    /// This can be used to manipulate the window into various special states, but note that not all states are supported on all systems.
    /// This function may return failure or an error if the platform implementation doesn't "understand" how to set the given style, but the return value here can't be used to determine if the state has actually been set.
    /// Any changes to the actual state of the view will arrive in later configure events.
    pub fn set_style(&self, style: ViewStyle) -> bool {
        unsafe { sys::puglSetViewStyle(self.view, style.bits()) == sys::PUGL_SUCCESS }
    }

    /// Activate a repeating timer event.
    ///
    /// This starts a timer which will send a [`Event::Timer`] event to view every `timeout` seconds.
    /// This can be used to perform some action in a view at a regular interval with relatively low frequency. Note that the frequency of timer events may be limited by how often [`World::update`] is called.
    /// If the given timer already exists, it is replaced.
    /// ### ID
    /// There is a platform-specific limit to the number of supported timers, and overhead associated with each, so applications should create only a few timers and perform several tasks in one if necessary.
    /// ### Timer Resolution
    /// Timers are not guaranteed to have a resolution better than 10ms (the maximum timer resolution on Windows)
    /// and may be rounded up if it is too short. On X11 and MacOS, a resolution of about 1ms can usually be relied on.
    pub fn start_timer(&self, id: TimerId, timeout: Duration) -> bool {
        unsafe { sys::puglStartTimer(self.view, id, timeout.as_secs_f64()) == sys::PUGL_SUCCESS }
    }

    /// Stop an active timer.
    pub fn stop_timer(&self, id: TimerId) -> bool {
        unsafe { sys::puglStopTimer(self.view, id) == sys::PUGL_SUCCESS }
    }

    /// Send a client event to a view via the window system.
    ///
    /// This can be used to send a custom message to a view, which is delivered via the window system and processed in the event loop as usual.
    /// Among other things, this makes it possible to wake up the event loop for any reason.
    pub fn send_client_event(&self, data: [usize; 2]) -> bool {
        unsafe {
            sys::puglSendEvent(self.view, &sys::PuglEvent {
                client: sys::PuglClientEvent {
                    type_: sys::PUGL_CLIENT,
                    flags: sys::PUGL_IS_SEND_EVENT,
                    data1: data[0],
                    data2: data[1],
                },
            }) == sys::PUGL_SUCCESS
        }
    }

    /// Send a close event to the event handler.
    pub fn send_close_event(&self) -> bool {
        unsafe {
            sys::puglSendEvent(self.view, &sys::PuglEvent {
                any: sys::PuglAnyEvent {
                    type_: sys::PUGL_CLOSE,
                    flags: sys::PUGL_IS_SEND_EVENT,
                },
            }) == sys::PUGL_SUCCESS
        }
    }

    /// Raise the window to the top of the application's stack.
    ///
    /// This is the normal "well-behaved" way to show and raise the window, which should be used in most cases.
    pub fn show(&self) -> bool {
        unsafe { sys::puglShow(self.view, sys::PUGL_SHOW_RAISE) == sys::PUGL_SUCCESS }
    }

    /// Realize and show the window without intentionally raising it.
    ///
    /// This will weakly "show" the window but without making any effort to raise it. Depending on the platform or system configuration, the window may be raised above some others regardless.
    pub fn show_passive(&self) -> bool {
        unsafe { sys::puglShow(self.view, sys::PUGL_SHOW_PASSIVE) == sys::PUGL_SUCCESS }
    }

    /// Aggressively force the window to be raised to the top.
    ///
    /// This will attempt to raise the window to the top, even if this isn't the active application, or if doing so would otherwise go against the platform's guidelines.
    /// This generally shouldn't be used, and isn't guaranteed to work. On modern Windows systems, the active application must explicitly grant permission for others to steal the foreground from it.
    pub fn show_aggressive(&self) -> bool {
        unsafe { sys::puglShow(self.view, sys::PUGL_SHOW_FORCE_RAISE) == sys::PUGL_SUCCESS }
    }

    /// Hide the current window.
    ///
    /// This will hide the window, but not destroy it. The window can be shown again with `show()`, `show_passive()` or `show_aggressive()`.
    pub fn hide(&self) {
        unsafe {
            sys::puglHide(self.view);
        }
    }

    /// Request a redisplay for the entire view.
    ///
    /// This will cause an expose event to be dispatched later. If called from within the event handler, the expose should arrive at the end of the current event loop iteration, though this is not strictly guaranteed on all platforms.
    /// If called elsewhere, an expose will be enqueued to be processed in the next event loop iteration.
    pub fn obscure_view(&self) {
        unsafe {
            sys::puglObscureView(self.view);
        }
    }

    /// "Obscure" a region so it will be exposed in the next render.
    ///
    /// This will cause an expose event to be dispatched later. If called from within the event handler, the expose should arrive at the end of the current event loop iteration, though this is not strictly guaranteed on all platforms.
    /// If called elsewhere, an expose will be enqueued to be processed in the next event loop iteration.
    /// The region is clamped to the size of the view if necessary.
    pub fn obscure_region(&self, rect: Rect) {
        unsafe {
            sys::puglObscureRegion(self.view, rect.x, rect.y, rect.w, rect.h);
        }
    }

    /// Grab the keyboard input focus.
    ///
    /// Note that this will fail if the view is not mapped and so should not, for example, be called immediately after show().
    pub fn grab_focus(&self) {
        unsafe {
            sys::puglGrabFocus(self.view);
        }
    }

    /// Return whether the view has the keyboard input focus
    pub fn has_focus(&self) -> bool {
        unsafe { sys::puglHasFocus(self.view) }
    }

    /// Returns the current position of the view in screen coordinates with an upper left origin
    pub fn position(&self) -> (i32, i32) {
        unsafe {
            let point = sys::puglGetPositionHint(self.view, sys::PUGL_CURRENT_POSITION);
            (point.x as i32, point.y as i32)
        }
    }

    /// Returns the current size of the view in (physical) pixels
    pub fn size(&self) -> (u32, u32) {
        unsafe {
            let size = sys::puglGetSizeHint(self.view, sys::PUGL_CURRENT_SIZE);
            (size.width as u32, size.height as u32)
        }
    }

    /// Returns the associated world instance
    pub fn world(&self) -> &World {
        self.world.as_world()
    }

    /// Return the parent window this view, if any
    pub fn parent(&self) -> Option<ViewParent> {
        unsafe {
            let parent = sys::puglGetParent(self.view);
            if parent != 0 {
                Some(ViewParent::Embedding(NativeView { ptr: parent }))
            } else {
                let parent = sys::puglGetTransientParent(self.view);
                if parent != 0 {
                    Some(ViewParent::Transient(NativeView { ptr: parent }))
                } else {
                    None
                }
            }
        }
    }

    /// Returns the native window handle
    pub fn native(&self) -> NativeView {
        unsafe {
            NativeView {
                ptr: sys::puglGetNativeView(self.view),
            }
        }
    }

    /// Returns the title of the window
    pub fn title(&self) -> String {
        unsafe {
            let title = sys::puglGetViewString(self.view, sys::PUGL_WINDOW_TITLE);
            if title.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(title)
                    .to_string_lossy()
                    .into_owned()
            }
        }
    }

    /// Return the current style of the view
    pub fn style(&self) -> ViewStyle {
        unsafe { ViewStyle::from_bits_truncate(sys::puglGetViewStyle(self.view)) }
    }

    /// Return true if the view is currently visible
    pub fn is_visible(&self) -> bool {
        unsafe { sys::puglGetVisible(self.view) }
    }

    /// Return the scale factor of the view.
    ///
    /// This factor describe how large UI elements (especially text) should be compared to "normal".
    /// For example, 2.0 means the UI should be drawn twice as large.
    /// "Normal" is loosely defined, but means a good size on a "standard DPI" display (around 96 DPI).
    /// In other words, the scale 1.0 should have text that is reasonably sized on a 96 DPI display, and the scale 2.0 should have text twice that large.
    pub fn system_scale(&self) -> f64 {
        unsafe { sys::puglGetScaleFactor(self.view) }
    }

    /// Set the clipboard contents.
    ///
    /// This sets the system clipboard contents, which can be retrieved with [`View::paste_clipboard`] or pasted into other applications.
    ///
    /// For now only text data is supported by the `pugl-rs` (and `pugl` itself supports only text data on windows)
    pub fn copy_clipboard(&self, string: &str) -> bool {
        unsafe {
            sys::puglSetClipboard(
                self.view,
                c"text/plain".as_ptr(),
                string.as_ptr() as _,
                string.len(),
            ) == sys::PUGL_SUCCESS
        }
    }

    /// Get the clipboard contents.
    pub fn paste_clipboard(&self) -> bool {
        unsafe { sys::puglPaste(self.view) == sys::PUGL_SUCCESS }
    }

    unsafe fn from_raw(view: *mut sys::PuglView) -> ManuallyDrop<View<B>> {
        unsafe {
            ManuallyDrop::new(Self {
                view,
                world: WorldInner::from_raw(sys::puglGetWorld(view)),
                phantom: PhantomData,
            })
        }
    }
}

impl<B: Backend> Drop for View<B> {
    fn drop(&mut self) {
        unsafe {
            sys::puglFreeView(self.view);
        }
    }
}

/// View realization error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewError {
    /// Invalid view configuration
    BadConfig,
    /// Invalid or missing backend
    BadBackend,
    /// Backend initialization failed
    BackendInit,
    /// System class registration failed
    ClassRegister,
    /// System view realization failed
    OsRealize,
    /// Failed to create drawing context
    CreateContext,
    /// Failed to set pixel format
    SetPixelFormat,
    /// Failed to allocate memory
    OutOfMemory,
    /// Unknown error
    Unknown,
}

impl std::error::Error for ViewError {}
impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendInit => write!(f, "backend initialization failed"),
            Self::BadBackend => write!(f, "invalid backend"),
            Self::BadConfig => write!(f, "invalid configuration"),
            Self::ClassRegister => write!(f, "failed to register class"),
            Self::CreateContext => write!(f, "failed to create context"),
            Self::OsRealize => write!(f, "failed to create os window"),
            Self::SetPixelFormat => write!(f, "failed to set pixel format"),
            Self::OutOfMemory => write!(f, "out of memory"),
            Self::Unknown => write!(f, "unknown error"),
        }
    }
}

impl<B: Backend> fmt::Debug for View<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("View")
            .field("style", &self.style())
            .field("visible", &self.is_visible())
            .field("position", &self.position())
            .field("size", &self.size())
            .field("title", &self.title())
            .field("parent", &self.parent())
            .field("native", &self.native())
            .field("system_scale", &self.system_scale())
            .finish()
    }
}

impl<B: Backend> fmt::Debug for UnrealizedView<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnrealizedView")
            .field("title", &self.title())
            .field("parent", &self.parent())
            .field("system_scale", &self.system_scale())
            .finish()
    }
}

/// double boxing to make it ffi safe :c
type EventHandler<B> = Mutex<Box<dyn FnMut(&View<B>, Event<B>) + Send>>;

unsafe extern "C" fn event_handler<B: Backend>(
    view: *mut sys::PuglView,
    raw: *const sys::PuglEvent,
) -> sys::PuglStatus {
    unsafe {
        if let Some(event) = Event::<B>::process(view, raw) {
            let handle = sys::puglGetHandle(view);
            if !handle.is_null() {
                let handler = &mut *(handle as *mut EventHandler<B>);
                let view = View::from_raw(view);

                handler.lock().unwrap()(&view, event);

                if (*raw).type_ == sys::PUGL_UNREALIZE {
                    drop(Box::from_raw(handle as *mut EventHandler<B>));
                }
            }
        }

        sys::PUGL_SUCCESS
    }
}
