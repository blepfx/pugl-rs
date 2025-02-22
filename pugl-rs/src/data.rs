use crate::{Backend, sys};
use std::ptr::addr_of;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Modifiers: u32 {
        const SHIFT = sys::PUGL_MOD_SHIFT;
        const CTRL = sys::PUGL_MOD_CTRL;
        const ALT = sys::PUGL_MOD_ALT;
        const SUPER = sys::PUGL_MOD_SUPER;
        const NUM_LOCK = sys::PUGL_MOD_NUM_LOCK;
        const CAPS_LOCK = sys::PUGL_MOD_CAPS_LOCK;
        const SCROLL_LOCK = sys::PUGL_MOD_SCROLL_LOCK;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct ViewStyle: u32 {
        const MAPPED = sys::PUGL_VIEW_STYLE_MAPPED;
        const MODAL = sys::PUGL_VIEW_STYLE_MODAL;
        const ABOVE = sys::PUGL_VIEW_STYLE_ABOVE;
        const BELOW = sys::PUGL_VIEW_STYLE_BELOW;
        const HIDDEN = sys::PUGL_VIEW_STYLE_HIDDEN;
        const TALL = sys::PUGL_VIEW_STYLE_TALL;
        const WIDE = sys::PUGL_VIEW_STYLE_WIDE;
        const FULLSCREEN = sys::PUGL_VIEW_STYLE_FULLSCREEN;
        const RESIZING = sys::PUGL_VIEW_STYLE_RESIZING;
        const DEMANDING = sys::PUGL_VIEW_STYLE_DEMANDING;
    }
}

pub type TimerId = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CrossingMode {
    Normal,
    Grab,
    Ungrab,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum MouseCursor {
    #[default]
    Arrow,
    Caret,
    Crosshair,
    Hand,
    NotAllowed,
    Scroll,
    ResizeWE,
    ResizeNS,
    ResizeNWSE,
    ResizeNESW,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum ViewType {
    #[default]
    Normal,
    Utility,
    Dialog,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
    Smooth,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Key {
    None,
    Char(char),
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Left,
    Up,
    Right,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    ShiftL,
    ShiftR,
    CtrlL,
    CtrlR,
    AltL,
    AltR,
    SuperL,
    SuperR,
    Menu,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter,
    NumpadEqual,
    NumpadUp,
    NumpadDown,
    NumpadLeft,
    NumpadRight,
    NumpadHome,
    NumpadEnd,
    NumpadPageUp,
    NumpadPageDown,
    NumpadInsert,
    NumpadDelete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventInput {
    pub time: f64,
    pub x: f64,
    pub y: f64,
    pub root_x: f64,
    pub root_y: f64,
    pub mods: Modifiers,
    pub hint: bool,
}

#[derive(Debug, PartialEq)]
pub enum Event<'a, B: Backend> {
    /// View resize or move event.
    ///
    /// A configure event is sent whenever the view is resized or moved.  
    /// When a configure event is received, the graphics context is active but not set up for drawing.  
    /// For example, it is valid to adjust the OpenGL viewport or otherwise configure the context,
    /// but not to draw anything.
    Configure {
        rect: Rect,
        style: ViewStyle,
    },

    Realize {
        backend: B::SetupContext<'a>,
    },

    Unrealize {
        backend: B::SetupContext<'a>,
    },

    /// Recursive loop enter event.
    ///
    /// This event is sent when the window system enters a recursive loop.  
    /// The main loop will be stalled and no expose events will be received while in the recursive loop.  
    /// To give the application full control, Pugl does not do any special handling of this situation,
    /// but this event can be used to install a timer to perform continuous actions (such as drawing) on platforms that do this.
    /// - MacOS: A recursive loop is entered while the window is being live resized.
    /// - Windows: A recursive loop is entered while the window is being live resized or the menu is shown.
    /// - X11: A recursive loop is never entered and the event loop runs as usual while the view is being resized.
    EnterLoop,

    /// Recursive loop leave event.
    ///
    /// This event is sent after a loop enter event when the recursive loop is finished and normal iteration will continue
    /// See `Event::EnterLoop` for more info.
    LeaveLoop,

    /// View close event.
    ///
    /// This event is sent when the view is to be closed, for example when the user clicks the close button.
    Close,

    /// View update event.
    /// This event is sent to every view near the end of a main loop iteration when any pending exposures are about to be redrawn.  
    /// It is typically used to mark regions to expose with `View::obscure_view` or `View::obscure_region`.  
    /// For example, to continuously animate, obscure the view when an update event is received, and it will receive an expose event shortly afterwards.
    Update,

    /// Expose event for when a region must be redrawn.
    ///
    /// When an expose event is received, the graphics context is active, and the view must draw the entire specified region.  
    /// The contents of the region are undefined, there is no preservation of anything drawn previously.
    Expose {
        backend: B::DrawContext<'a>,
        rect: Rect,
    },

    /// Keyboard focus event.
    ///
    /// This event is sent whenever the view gains the keyboard focus.  
    /// The view with the keyboard focus will receive any key press or release events.
    FocusIn {
        mode: CrossingMode,
    },

    /// Keyboard focus event.
    ///
    /// This event is sent whenever the view loses the keyboard focus.
    /// The view with the keyboard focus will receive any key press or release events.
    FocusOut {
        mode: CrossingMode,
    },

    /// Key press event.
    ///
    /// This event represents low-level key presses.  
    /// This can be used for "direct" keyboard handling like key bindings, but must not be interpreted as text input.
    /// Keys are represented portably as Unicode code points, using the "natural" code point for the key where possible
    ///
    /// The `key` field is the code for the pressed key, without any modifiers applied.  
    /// For example, a press or release of the 'A' key will have `key` 97 ('a') regardless of whether shift or control are being held.
    ///
    /// Alternatively, the raw `keycode` can be used to work directly with physical keys,
    /// but note that this value is not portable and differs between platforms and hardware.
    KeyPress {
        input: EventInput,
        keycode: u32,
        key: Key,
    },

    /// Key press event.
    ///
    /// This event represents low-level key preleases.  
    /// This can be used for "direct" keyboard handling like key bindings, but must not be interpreted as text input.
    /// Keys are represented portably as Unicode code points, using the "natural" code point for the key where possible
    ///
    /// The `key` field is the code for the pressed key, without any modifiers applied.  
    /// For example, a press or release of the 'A' key will have `key` 97 ('a') regardless of whether shift or control are being held.
    ///
    /// Alternatively, the raw `keycode` can be used to work directly with physical keys,
    /// but note that this value is not portable and differs between platforms and hardware.
    KeyRelease {
        input: EventInput,
        keycode: u32,
        key: Key,
    },

    /// Character input event.
    ///
    /// This event represents text input, usually as the result of a key press.  
    /// The text is given both as a Unicode character code and a UTF-8 string.
    /// Note that this event is generated by the platform's input system, so there is not necessarily a direct correspondence between text events and physical key presses.  
    /// For example, with some input methods a sequence of several key presses will generate a single character.
    KeyText {
        input: EventInput,
        keycode: u32,
        text: &'a str,
    },

    /// Pointer enter event.
    ///
    /// This event is sent when the pointer enters the view.  
    /// This can happen for several reasons, as described by the `mode` field.
    PointerEnter {
        input: EventInput,
        mode: CrossingMode,
    },

    /// Pointer leave event.
    ///
    /// This event is sent when the pointer leaves the view.
    /// This can happen for several reasons, as described by the `mode` field.
    PointerLeave {
        input: EventInput,
        mode: CrossingMode,
    },

    /// Pointer motion event.
    PointerMotion {
        input: EventInput,
    },

    /// Button press event.
    ButtonPress {
        input: EventInput,
        button: MouseButton,
    },

    /// Button release event.
    ButtonRelease {
        input: EventInput,
        button: MouseButton,
    },

    /// Scroll event.
    ///
    /// The scroll distance is expressed in "lines", an arbitrary unit that corresponds to a single tick of a detented mouse wheel.  
    /// For example, `dy` = 1.0 scrolls 1 line up.  
    /// Some systems and devices support finer resolution and/or higher values for fast scrolls, so programs should handle any value gracefully.
    Scroll {
        input: EventInput,
        direction: ScrollDirection,
        dx: f64,
        dy: f64,
    },

    /// Timer event.
    ///
    /// This event is sent at the regular interval specified in the call to `View::start_timer` that activated it.
    /// The `id` is the application-specific ID given to `View::start_timer` which distinguishes this timer from others.  
    /// It should always be checked in the event handler, even in applications that register only one timer.
    Timer {
        id: TimerId,
    },

    /// A custom client event.
    ///
    /// See `View::send_client_event` for more info.
    Client {
        data: [usize; 2],
    },
    //TODO: data offer
}

impl MouseCursor {
    pub fn into_raw(self) -> sys::PuglCursor {
        match self {
            MouseCursor::Arrow => sys::PUGL_CURSOR_ARROW,
            MouseCursor::Caret => sys::PUGL_CURSOR_CARET,
            MouseCursor::Crosshair => sys::PUGL_CURSOR_CROSSHAIR,
            MouseCursor::Hand => sys::PUGL_CURSOR_HAND,
            MouseCursor::NotAllowed => sys::PUGL_CURSOR_NO,
            MouseCursor::Scroll => sys::PUGL_CURSOR_ALL_SCROLL,
            MouseCursor::ResizeWE => sys::PUGL_CURSOR_LEFT_RIGHT,
            MouseCursor::ResizeNS => sys::PUGL_CURSOR_UP_DOWN,
            MouseCursor::ResizeNWSE => sys::PUGL_CURSOR_UP_LEFT_DOWN_RIGHT,
            MouseCursor::ResizeNESW => sys::PUGL_CURSOR_UP_RIGHT_DOWN_LEFT,
        }
    }
}

impl ViewType {
    pub fn into_raw(self) -> u32 {
        match self {
            ViewType::Normal => sys::PUGL_VIEW_TYPE_NORMAL,
            ViewType::Utility => sys::PUGL_VIEW_TYPE_UTILITY,
            ViewType::Dialog => sys::PUGL_VIEW_TYPE_DIALOG,
        }
    }
}

impl ScrollDirection {
    pub fn from_raw(raw: sys::PuglScrollDirection) -> Self {
        match raw {
            sys::PUGL_SCROLL_UP => ScrollDirection::Up,
            sys::PUGL_SCROLL_DOWN => ScrollDirection::Down,
            sys::PUGL_SCROLL_LEFT => ScrollDirection::Left,
            sys::PUGL_SCROLL_RIGHT => ScrollDirection::Right,
            _ => ScrollDirection::Smooth,
        }
    }
}

impl CrossingMode {
    pub fn from_raw(raw: sys::PuglCrossingMode) -> Self {
        match raw {
            sys::PUGL_CROSSING_GRAB => CrossingMode::Grab,
            sys::PUGL_CROSSING_UNGRAB => CrossingMode::Ungrab,
            _ => CrossingMode::Normal,
        }
    }
}

impl MouseButton {
    pub fn from_raw(raw: u32) -> Self {
        match raw {
            0 => MouseButton::Left,
            1 => MouseButton::Right,
            2 => MouseButton::Middle,
            3 => MouseButton::Back,
            4 => MouseButton::Forward,
            _ => MouseButton::Other(raw),
        }
    }
}

impl Key {
    pub fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Key::None,
            sys::PUGL_KEY_ALT_L => Key::AltL,
            sys::PUGL_KEY_ALT_R => Key::AltR,
            sys::PUGL_KEY_CTRL_L => Key::CtrlL,
            sys::PUGL_KEY_CTRL_R => Key::CtrlR,
            sys::PUGL_KEY_SHIFT_L => Key::ShiftL,
            sys::PUGL_KEY_SHIFT_R => Key::ShiftR,
            sys::PUGL_KEY_SUPER_L => Key::SuperL,
            sys::PUGL_KEY_SUPER_R => Key::SuperR,

            sys::PUGL_KEY_CAPS_LOCK => Key::CapsLock,
            sys::PUGL_KEY_NUM_LOCK => Key::NumLock,
            sys::PUGL_KEY_PAUSE => Key::Pause,
            sys::PUGL_KEY_PRINT_SCREEN => Key::PrintScreen,
            sys::PUGL_KEY_SCROLL_LOCK => Key::ScrollLock,
            sys::PUGL_KEY_PAGE_DOWN => Key::PageDown,
            sys::PUGL_KEY_PAGE_UP => Key::PageUp,

            sys::PUGL_KEY_END => Key::End,
            sys::PUGL_KEY_MENU => Key::Menu,
            sys::PUGL_KEY_HOME => Key::Home,
            sys::PUGL_KEY_INSERT => Key::Insert,

            sys::PUGL_KEY_F1 => Key::F1,
            sys::PUGL_KEY_F2 => Key::F2,
            sys::PUGL_KEY_F3 => Key::F3,
            sys::PUGL_KEY_F4 => Key::F4,
            sys::PUGL_KEY_F5 => Key::F5,
            sys::PUGL_KEY_F6 => Key::F6,
            sys::PUGL_KEY_F7 => Key::F7,
            sys::PUGL_KEY_F8 => Key::F8,
            sys::PUGL_KEY_F9 => Key::F9,
            sys::PUGL_KEY_F10 => Key::F10,
            sys::PUGL_KEY_F11 => Key::F11,
            sys::PUGL_KEY_F12 => Key::F12,

            sys::PUGL_KEY_DOWN => Key::Down,
            sys::PUGL_KEY_LEFT => Key::Left,
            sys::PUGL_KEY_RIGHT => Key::Right,
            sys::PUGL_KEY_UP => Key::Up,

            sys::PUGL_KEY_PAD_0 => Key::Numpad0,
            sys::PUGL_KEY_PAD_1 => Key::Numpad1,
            sys::PUGL_KEY_PAD_2 => Key::Numpad2,
            sys::PUGL_KEY_PAD_3 => Key::Numpad3,
            sys::PUGL_KEY_PAD_4 => Key::Numpad4,
            sys::PUGL_KEY_PAD_5 => Key::Numpad5,
            sys::PUGL_KEY_PAD_6 => Key::Numpad6,
            sys::PUGL_KEY_PAD_7 => Key::Numpad7,
            sys::PUGL_KEY_PAD_8 => Key::Numpad8,
            sys::PUGL_KEY_PAD_9 => Key::Numpad9,
            sys::PUGL_KEY_PAD_ADD => Key::NumpadAdd,
            sys::PUGL_KEY_PAD_SUBTRACT => Key::NumpadSubtract,
            sys::PUGL_KEY_PAD_MULTIPLY => Key::NumpadMultiply,
            sys::PUGL_KEY_PAD_DIVIDE => Key::NumpadDivide,
            sys::PUGL_KEY_PAD_DECIMAL => Key::NumpadDecimal,
            sys::PUGL_KEY_PAD_ENTER => Key::NumpadEnter,
            sys::PUGL_KEY_PAD_EQUAL => Key::NumpadEqual,
            sys::PUGL_KEY_PAD_UP => Key::NumpadUp,
            sys::PUGL_KEY_PAD_DOWN => Key::NumpadDown,
            sys::PUGL_KEY_PAD_LEFT => Key::NumpadLeft,
            sys::PUGL_KEY_PAD_RIGHT => Key::NumpadRight,
            sys::PUGL_KEY_PAD_HOME => Key::NumpadHome,
            sys::PUGL_KEY_PAD_END => Key::NumpadEnd,
            sys::PUGL_KEY_PAD_PAGE_UP => Key::NumpadPageUp,
            sys::PUGL_KEY_PAD_PAGE_DOWN => Key::NumpadPageDown,
            sys::PUGL_KEY_PAD_INSERT => Key::NumpadInsert,
            sys::PUGL_KEY_PAD_DELETE => Key::NumpadDelete,

            _ => match char::from_u32(raw) {
                Some(char) => Key::Char(char),
                None => Key::None,
            },
        }
    }
}

impl<'a, B: Backend> Event<'a, B> {
    pub unsafe fn from_raw(view: *mut sys::PuglView, event: *const sys::PuglEvent) -> Option<Self> {
        unsafe {
            Some(match (*event).type_ {
                sys::PUGL_REALIZE => Event::Realize {
                    backend: B::setup(view, crate::private::Private),
                },

                sys::PUGL_UNREALIZE => Event::Unrealize {
                    backend: B::setup(view, crate::private::Private),
                },

                sys::PUGL_LOOP_ENTER => Event::EnterLoop,
                sys::PUGL_LOOP_LEAVE => Event::LeaveLoop,
                sys::PUGL_CONFIGURE => Event::Configure {
                    style: ViewStyle::from_bits_truncate((*event).configure.style),
                    rect: Rect {
                        x: (*event).configure.x as i32,
                        y: (*event).configure.y as i32,
                        w: (*event).configure.width as u32,
                        h: (*event).configure.height as u32,
                    },
                },
                sys::PUGL_CLOSE => Event::Close,
                sys::PUGL_UPDATE => Event::Update,
                sys::PUGL_EXPOSE => Event::Expose {
                    backend: B::draw(view, crate::private::Private),
                    rect: Rect {
                        x: (*event).expose.x as i32,
                        y: (*event).expose.y as i32,
                        w: (*event).expose.width as u32,
                        h: (*event).expose.height as u32,
                    },
                },
                sys::PUGL_FOCUS_IN => Event::FocusIn {
                    mode: CrossingMode::from_raw((*event).focus.mode),
                },
                sys::PUGL_FOCUS_OUT => Event::FocusOut {
                    mode: CrossingMode::from_raw((*event).focus.mode),
                },
                sys::PUGL_KEY_PRESS => Event::KeyPress {
                    input: EventInput {
                        time: (*event).key.time,
                        x: (*event).key.x,
                        y: (*event).key.y,
                        root_x: (*event).key.xRoot,
                        root_y: (*event).key.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).key.state),
                        hint: ((*event).key.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    keycode: (*event).key.keycode,
                    key: Key::from_raw((*event).key.key),
                },
                sys::PUGL_KEY_RELEASE => Event::KeyRelease {
                    input: EventInput {
                        time: (*event).key.time,
                        x: (*event).key.x,
                        y: (*event).key.y,
                        root_x: (*event).key.xRoot,
                        root_y: (*event).key.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).key.state),
                        hint: ((*event).key.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    keycode: (*event).key.keycode,
                    key: Key::from_raw((*event).key.key),
                },
                sys::PUGL_TEXT => Event::KeyText {
                    input: EventInput {
                        time: (*event).key.time,
                        x: (*event).key.x,
                        y: (*event).key.y,
                        root_x: (*event).key.xRoot,
                        root_y: (*event).key.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).key.state),
                        hint: ((*event).key.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    keycode: (*event).key.keycode,
                    text: {
                        let bytes = &*addr_of!((*event).text.string).cast::<[u8; 8]>();
                        let len = bytes.iter().position(|&b| b == 0).unwrap_or(8);
                        std::str::from_utf8(&bytes[..len]).ok()?
                    },
                },
                sys::PUGL_POINTER_IN => Event::PointerEnter {
                    input: EventInput {
                        time: (*event).crossing.time,
                        x: (*event).crossing.x,
                        y: (*event).crossing.y,
                        root_x: (*event).crossing.xRoot,
                        root_y: (*event).crossing.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).crossing.state),
                        hint: ((*event).crossing.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    mode: CrossingMode::from_raw((*event).crossing.mode),
                },
                sys::PUGL_POINTER_OUT => Event::PointerLeave {
                    input: EventInput {
                        time: (*event).crossing.time,
                        x: (*event).crossing.x,
                        y: (*event).crossing.y,
                        root_x: (*event).crossing.xRoot,
                        root_y: (*event).crossing.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).crossing.state),
                        hint: ((*event).crossing.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    mode: CrossingMode::from_raw((*event).crossing.mode),
                },
                sys::PUGL_BUTTON_PRESS => Event::ButtonPress {
                    input: EventInput {
                        time: (*event).button.time,
                        x: (*event).button.x,
                        y: (*event).button.y,
                        root_x: (*event).button.xRoot,
                        root_y: (*event).button.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).button.state),
                        hint: ((*event).button.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    button: MouseButton::from_raw((*event).button.button),
                },
                sys::PUGL_BUTTON_RELEASE => Event::ButtonRelease {
                    input: EventInput {
                        time: (*event).button.time,
                        x: (*event).button.x,
                        y: (*event).button.y,
                        root_x: (*event).button.xRoot,
                        root_y: (*event).button.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).button.state),
                        hint: ((*event).button.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    button: MouseButton::from_raw((*event).button.button),
                },
                sys::PUGL_MOTION => Event::PointerMotion {
                    input: EventInput {
                        time: (*event).motion.time,
                        x: (*event).motion.x,
                        y: (*event).motion.y,
                        root_x: (*event).motion.xRoot,
                        root_y: (*event).motion.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).motion.state),
                        hint: ((*event).motion.flags & sys::PUGL_IS_HINT) != 0,
                    },
                },
                sys::PUGL_SCROLL => Event::Scroll {
                    input: EventInput {
                        time: (*event).scroll.time,
                        x: (*event).scroll.x,
                        y: (*event).scroll.y,
                        root_x: (*event).scroll.xRoot,
                        root_y: (*event).scroll.yRoot,
                        mods: Modifiers::from_bits_truncate((*event).scroll.state),
                        hint: ((*event).scroll.flags & sys::PUGL_IS_HINT) != 0,
                    },
                    dx: (*event).scroll.dx,
                    dy: (*event).scroll.dy,
                    direction: ScrollDirection::from_raw((*event).scroll.direction),
                },

                sys::PUGL_CLIENT => Event::Client {
                    data: [(*event).client.data1, (*event).client.data2],
                },

                sys::PUGL_TIMER => Event::Timer {
                    id: (*event).timer.id,
                },

                _ => return None,
            })
        }
    }
}
