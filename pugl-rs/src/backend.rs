use crate::sys;
use std::fmt::Debug;

pub(crate) mod private {
    pub struct Private;
}

pub trait Backend {
    type DrawContext<'a>: Debug;
    type SetupContext<'a>: Debug;

    unsafe fn install(self, view: *mut sys::PuglView, _private: private::Private);

    unsafe fn setup<'a>(
        view: *mut sys::PuglView,
        _private: private::Private,
    ) -> Self::SetupContext<'a>;

    unsafe fn draw<'a>(
        view: *mut sys::PuglView,
        _private: private::Private,
    ) -> Self::DrawContext<'a>;
}

impl Backend for () {
    type DrawContext<'a> = ();
    type SetupContext<'a> = ();

    unsafe fn install(self, view: *mut sys::PuglView, _private: private::Private) {
        unsafe {
            sys::puglSetBackend(view, sys::puglStubBackend());
        }
    }

    unsafe fn setup<'a>(
        _view: *mut pugl_rs_sys::PuglView,
        _private: private::Private,
    ) -> Self::SetupContext<'a> {
        ()
    }

    unsafe fn draw<'a>(
        _view: *mut pugl_rs_sys::PuglView,
        _private: private::Private,
    ) -> Self::DrawContext<'a> {
        ()
    }
}
