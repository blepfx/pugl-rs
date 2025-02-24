use crate::sys;

/// Represents a graphics backend for a view.
///
/// Available backends are:
/// - `()` - stub backend, no drawing
/// - `OpenGl` - OpenGL backend, gated behind the `opengl` feature
pub trait Backend: std::fmt::Debug {
    /// The context used for drawing on the view. Can be accessed via `Event::Expose`.
    type DrawContext<'a>: std::fmt::Debug;

    /// The context used for setting up the view.
    ///
    /// No drawing operations are allowed inside the setup context scope.
    /// This context can only be used for backend resource creation or deletion.
    type SetupContext<'a>: std::fmt::Debug;

    #[doc(hidden)]
    unsafe fn install(self, view: *mut sys::PuglView, _: crate::private::Private);

    #[doc(hidden)]
    unsafe fn setup<'a>(
        view: *mut sys::PuglView,
        _: crate::private::Private,
    ) -> Self::SetupContext<'a>;

    #[doc(hidden)]
    unsafe fn draw<'a>(
        view: *mut sys::PuglView,
        _: crate::private::Private,
    ) -> Self::DrawContext<'a>;
}

impl Backend for () {
    type DrawContext<'a> = ();
    type SetupContext<'a> = ();

    unsafe fn install(self, view: *mut sys::PuglView, _: crate::private::Private) {
        unsafe {
            sys::puglSetBackend(view, sys::puglStubBackend());
        }
    }

    unsafe fn setup<'a>(
        _view: *mut pugl_rs_sys::PuglView,
        _: crate::private::Private,
    ) -> Self::SetupContext<'a> {
        ()
    }

    unsafe fn draw<'a>(
        _view: *mut pugl_rs_sys::PuglView,
        _: crate::private::Private,
    ) -> Self::DrawContext<'a> {
        ()
    }
}

#[cfg(feature = "opengl")]
pub use opengl::*;

#[cfg(feature = "opengl")]
mod opengl {
    use super::*;
    use std::{
        ffi::{CStr, c_void},
        fmt,
        marker::PhantomData,
        ptr::null_mut,
    };

    #[derive(Copy, Clone, Debug)]
    pub enum OpenGlVersion {
        Core(u8, u8),
        Compat(u8, u8),
        ES(u8, u8),
    }

    #[derive(Clone, Debug)]
    pub struct OpenGl {
        pub version: OpenGlVersion,
        pub debug: bool,
        pub double_buffer: bool,
        pub swap_interval: Option<u8>,
        pub bits_red: u8,
        pub bits_green: u8,
        pub bits_blue: u8,
        pub bits_alpha: u8,
        pub bits_stencil: u8,
        pub bits_depth: u8,
        pub aa_buffers: Option<u8>,
        pub aa_samples: u8,
    }

    impl Default for OpenGl {
        fn default() -> Self {
            OpenGl {
                version: OpenGlVersion::Core(2, 0),
                debug: false,
                double_buffer: true,
                swap_interval: None,
                bits_red: 8,
                bits_green: 8,
                bits_blue: 8,
                bits_alpha: 8,
                bits_stencil: 0,
                bits_depth: 0,
                aa_buffers: None,
                aa_samples: 0,
            }
        }
    }

    pub struct OpenGlContext<'a> {
        phantom: PhantomData<&'a ()>,
        view: *mut sys::PuglView,
    }

    impl<'a> OpenGlContext<'a> {
        pub fn get_proc_address(&self, name: &CStr) -> *mut c_void {
            unsafe {
                sys::puglGetProcAddress(name.as_ptr())
                    .map(|x| x as *mut _)
                    .unwrap_or(null_mut())
            }
        }
    }

    impl<'a> fmt::Debug for OpenGlContext<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpenGlContext")
                .field("view", &self.view)
                .finish()
        }
    }

    impl Backend for OpenGl {
        type DrawContext<'a> = OpenGlContext<'a>;
        type SetupContext<'a> = OpenGlContext<'a>;

        unsafe fn install(self, view: *mut sys::PuglView, _: crate::private::Private) {
            unsafe {
                sys::puglSetBackend(view, sys::puglGlBackend());

                match self.version {
                    OpenGlVersion::Core(major, minor) => {
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_API, sys::PUGL_OPENGL_API);
                        sys::puglSetViewHint(
                            view,
                            sys::PUGL_CONTEXT_PROFILE,
                            sys::PUGL_OPENGL_CORE_PROFILE,
                        );
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MAJOR, major as _);
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MINOR, minor as _);
                    }
                    OpenGlVersion::Compat(major, minor) => {
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_API, sys::PUGL_OPENGL_API);
                        sys::puglSetViewHint(
                            view,
                            sys::PUGL_CONTEXT_PROFILE,
                            sys::PUGL_OPENGL_COMPATIBILITY_PROFILE,
                        );
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MAJOR, major as _);
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MINOR, minor as _);
                    }
                    OpenGlVersion::ES(major, minor) => {
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_API, sys::PUGL_OPENGL_ES_API);
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MAJOR, major as _);
                        sys::puglSetViewHint(view, sys::PUGL_CONTEXT_VERSION_MINOR, minor as _);
                    }
                }

                sys::puglSetViewHint(view, sys::PUGL_CONTEXT_DEBUG, self.debug as _);
                sys::puglSetViewHint(view, sys::PUGL_DOUBLE_BUFFER, self.double_buffer as _);

                sys::puglSetViewHint(view, sys::PUGL_RED_BITS, self.bits_red as _);
                sys::puglSetViewHint(view, sys::PUGL_GREEN_BITS, self.bits_green as _);
                sys::puglSetViewHint(view, sys::PUGL_BLUE_BITS, self.bits_blue as _);
                sys::puglSetViewHint(view, sys::PUGL_ALPHA_BITS, self.bits_alpha as _);
                sys::puglSetViewHint(view, sys::PUGL_DEPTH_BITS, self.bits_depth as _);
                sys::puglSetViewHint(view, sys::PUGL_STENCIL_BITS, self.bits_stencil as _);

                sys::puglSetViewHint(view, sys::PUGL_SAMPLES, self.aa_samples as _);

                if let Some(aa_buffers) = self.aa_buffers {
                    sys::puglSetViewHint(view, sys::PUGL_SAMPLE_BUFFERS, aa_buffers as _);
                }

                if let Some(swap_interval) = self.swap_interval {
                    sys::puglSetViewHint(view, sys::PUGL_SWAP_INTERVAL, swap_interval as _);
                }
            }
        }

        unsafe fn setup<'a>(
            view: *mut sys::PuglView,
            _: crate::private::Private,
        ) -> Self::SetupContext<'a> {
            OpenGlContext {
                phantom: PhantomData,
                view,
            }
        }

        unsafe fn draw<'a>(
            view: *mut sys::PuglView,
            _: crate::private::Private,
        ) -> Self::DrawContext<'a> {
            OpenGlContext {
                phantom: PhantomData,
                view,
            }
        }
    }
}
