use pugl_sys::*;
use std::ffi::{CStr, CString};

extern "C" fn event_handler(view: *mut PuglView, event: *const PuglEvent) -> PuglStatus {
    unsafe {
        if (*event).type_ == PUGL_CLOSE {
            std::process::exit(0);
        }

        if (*event).type_ == PUGL_UPDATE {
            puglObscureView(view);
        }

        if (*event).type_ == PUGL_EXPOSE {
            //TODO: implement
        }

        // Handle events here
        PUGL_SUCCESS
    }
}

fn main() {
    unsafe {
        let world = puglNewWorld(PUGL_PROGRAM, 0);

        puglSetWorldString(
            world,
            PUGL_CLASS_NAME,
            CString::new("waow".to_string()).unwrap().as_ptr() as *const _,
        );

        let view = puglNewView(world);

        puglSetViewString(
            view,
            PUGL_WINDOW_TITLE,
            CString::new("Window Demo".to_string()).unwrap().as_ptr() as *const _,
        );

        puglSetSizeHint(view, PUGL_DEFAULT_SIZE, 512, 512);
        puglSetSizeHint(view, PUGL_MIN_SIZE, 128, 128);
        puglSetSizeHint(view, PUGL_MAX_SIZE, 2048, 2048);
        puglSetBackend(view, puglVulkanBackend());

        puglSetViewHint(view, PUGL_CONTEXT_DEBUG, 1);
        puglSetViewHint(view, PUGL_RESIZABLE, 1);
        puglSetViewHint(view, PUGL_SAMPLES, 1);
        puglSetViewHint(view, PUGL_DOUBLE_BUFFER, 1);
        puglSetViewHint(view, PUGL_SWAP_INTERVAL, 1);
        puglSetViewHint(view, PUGL_IGNORE_KEY_REPEAT, 0);

        puglSetEventFunc(view, Some(event_handler));

        let err = puglRealize(view);
        println!("err: {:?} {:?}", err, CStr::from_ptr(puglStrerror(err)));
        puglShow(view, PUGL_SHOW_RAISE);

        loop {
            if puglUpdate(world, 0.0) != 0 {
                break;
            }
        }

        puglFreeView(view);
        puglFreeWorld(world);
    }
}
