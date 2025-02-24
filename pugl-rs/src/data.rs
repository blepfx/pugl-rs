use crate::{Backend, sys};
use std::{ffi::CStr, ptr::addr_of, slice::from_raw_parts, str::from_utf8};

// doc only import
#[allow(unused_imports)]
use crate::{View, World};

bitflags::bitflags! {
    /// Keyboard modifier flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Modifiers: u32 {
        /// Shift held
        const SHIFT = sys::PUGL_MOD_SHIFT;
        /// Control held
        const CTRL = sys::PUGL_MOD_CTRL;
        /// Alt/Option held
        const ALT = sys::PUGL_MOD_ALT;
        /// Super/Command/Windows key held
        const SUPER = sys::PUGL_MOD_SUPER;
        /// Num lock active
        const NUM_LOCK = sys::PUGL_MOD_NUM_LOCK;
        /// Caps lock active
        const CAPS_LOCK = sys::PUGL_MOD_CAPS_LOCK;
        /// Scroll lock active
        const SCROLL_LOCK = sys::PUGL_MOD_SCROLL_LOCK;
    }
}

bitflags::bitflags! {
    /// View style flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct ViewStyle: u32 {
        /// View is mapped to a real window and potentially visible
        const MAPPED = sys::PUGL_VIEW_STYLE_MAPPED;
        /// View is modal, typically a dialog box of its transient parent
        const MODAL = sys::PUGL_VIEW_STYLE_MODAL;
        /// View should be above most others
        const ABOVE = sys::PUGL_VIEW_STYLE_ABOVE;
        /// View should be below most others
        const BELOW = sys::PUGL_VIEW_STYLE_BELOW;
        /// View is minimized, shaded, or otherwise invisible
        const HIDDEN = sys::PUGL_VIEW_STYLE_HIDDEN;
        /// View is maximized to fill the screen vertically
        const TALL = sys::PUGL_VIEW_STYLE_TALL;
        /// View is maximized to fill the screen horizontally
        const WIDE = sys::PUGL_VIEW_STYLE_WIDE;
        /// View is enlarged to fill the entire screen with no decorations
        const FULLSCREEN = sys::PUGL_VIEW_STYLE_FULLSCREEN;
        /// View is currently being resized
        const RESIZING = sys::PUGL_VIEW_STYLE_RESIZING;
        /// View is ready for input or otherwise demanding attention
        const DEMANDING = sys::PUGL_VIEW_STYLE_DEMANDING;
    }
}

/// An application-specific timer identifier.
///
/// Used in [`Event::Timer`], [`View::start_timer`] and [`View::stop_timer`].
///
/// There is a platform-specific limit to the number of supported timers, and overhead associated with each,
/// so applications should create only a few timers and perform several tasks in one if necessary.
///
/// The `TimerId` is the application-specific ID given to [`View::start_timer`] which distinguishes this timer from others.  
/// It should always be checked in the event handler, even in applications that register only one timer.
pub type TimerId = usize;

/// Reason for [`Event::PointerIn`], [`Event::PointerOut`], [`Event::FocusIn`] or [`Event::FocusOut`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CrossingMode {
    /// Crossing due to a normal pointer motion
    Normal,
    /// Crossing due to a grab
    Grab,
    /// Crossing due to a grab release
    Ungrab,
}

/// An arbitrary rectangle in (physical) pixel coordinates with top-left origin.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// Mouse cursor icon.
///
/// Used in [`View::set_cursor`].
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

/// A view type.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum ViewType {
    /// A normal top-level window
    #[default]
    Normal,
    /// A utility window like a palette or toolbox
    Utility,
    /// A dialog window
    Dialog,
}

/// Mouse button.
///
/// Used in [`Event::ButtonPress`] and [`Event::ButtonRelease`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u32),
}

/// Scroll direction.
///
/// Describes the direction of a [`Event::Scroll`] along with whether the scroll is a "smooth" scroll.
/// The discrete directions are for devices like mouse wheels with constrained axes,
/// while a smooth scroll is for those with arbitrary scroll direction freedom, like some touchpads.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
    Smooth,
}

/// Keyboard key codepoints.
///
/// Keys are represented portably as Unicode code points, using the "natural" code point for the key where possible.
/// For example, the 'A' key is represented as 97 ('a') regardless of whether shift or control are being held.
///
/// This enum also contains special keys (like F-keys or arrow keys) that are not representable that way.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Key {
    /// A sentinel value for when no key/unknown key is pressed/released
    None,

    /// A character key without any modifiers applied.
    ///
    /// For example, a press or release of the 'A' key will have the value of 97 ('a') regardless of whether shift or control are being held.
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
    NumpadSeparator,
    NumpadClear,
}

/// Event data associated with a user input event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventInput {
    /// Time of the event. Use [`World::time`] to get the current time.
    pub time: f64,

    /// X coordinate of the event in view coordinates.
    pub x: f64,
    /// Y coordinate of the event in view coordinates.
    pub y: f64,

    /// X coordinate of the event in screen coordinates.
    pub root_x: f64,
    /// Y coordinate of the event in screen coordinates.
    pub root_y: f64,

    /// Keyboard modifiers active at the time of the event.
    pub mods: Modifiers,

    /// Whether the event is a hint (i.e. was not created by a _direct_ user input)
    pub hint: bool,
}

/// A view event.
#[derive(Debug)]
pub enum Event<'a, B: Backend> {
    /// View resize or move event.
    ///
    /// A configure event is sent whenever the view is resized or moved.  
    /// When a configure event is received, the graphics context is active but not set up for drawing.  
    /// For example, it is valid to adjust the OpenGL viewport or otherwise configure the context,
    /// but not to draw anything.
    Configure { rect: Rect, style: ViewStyle },

    /// View realize event.
    ///
    /// This event is sent when a view is realized before it is first displayed, with the graphics context entered.  
    /// This is typically used for setting up the graphics system, for example by loading OpenGL extensions.
    Realize { backend: B::SetupContext<'a> },

    /// View unrealize event.
    ///
    /// This event is the counterpart to [`Event::Realize`], and is sent when the view will no longer be displayed.  
    /// This is typically used for tearing down the graphics system, or otherwise freeing any resources allocated when the realize event was handled.
    Unrealize { backend: B::SetupContext<'a> },

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
    /// It is typically used to mark regions to expose with [`View::obscure_view`] or [`View::obscure_region`].  
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
    FocusIn { mode: CrossingMode },

    /// Keyboard focus event.
    ///
    /// This event is sent whenever the view loses the keyboard focus.
    /// The view with the keyboard focus will receive any key press or release events.
    FocusOut { mode: CrossingMode },

    /// Key press event. See [`Key`] for more info.
    ///
    /// This event represents low-level key presses.  
    /// This can be used for "direct" keyboard handling like key bindings, but must not be interpreted as text input.
    ///
    /// Alternatively, the raw `keycode` can be used to work directly with physical keys,
    /// but note that this value is not portable and differs between platforms and hardware.
    KeyPress {
        input: EventInput,
        keycode: u32,
        key: Key,
    },

    /// Key press event. See [`Key`] for more info.
    ///
    /// This event represents low-level key releases.  
    /// This can be used for "direct" keyboard handling like key bindings, but must not be interpreted as text input.
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
    ///
    /// Note that this event is generated by the platform's input system, so there is not necessarily a direct correspondence between text events and physical key presses.  
    /// For example, with some input methods a sequence of several key presses will generate a single character.
    ///
    /// Alternatively, the raw `keycode` can be used to work directly with physical keys,
    /// but note that this value is not portable and differs between platforms and hardware.
    KeyText {
        input: EventInput,
        keycode: u32,
        text: &'a str,
    },

    /// Pointer enter event.
    ///
    /// This event is sent when the pointer enters the view.  
    /// This can happen for several reasons, as described by the `mode` field.
    PointerIn {
        input: EventInput,
        mode: CrossingMode,
    },

    /// Pointer leave event.
    ///
    /// This event is sent when the pointer leaves the view.
    /// This can happen for several reasons, as described by the `mode` field.
    PointerOut {
        input: EventInput,
        mode: CrossingMode,
    },

    /// Pointer motion event.
    PointerMotion { input: EventInput },

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
    /// This event is sent at the regular interval specified in the call to [`View::start_timer`] that activated it.
    /// The `id` is the application-specific ID given to [`View::start_timer`] which distinguishes this timer from others.  
    /// It should always be checked in the event handler, even in applications that register only one timer.
    Timer { id: TimerId },

    /// A custom client event.
    ///
    /// See [`View::send_client_event`] for more info.
    Client { data: [usize; 2] },

    /// A clipboard paste event.
    ///
    /// This event is sent if the clipboard contained text data at the time [`View::paste_clipboard`] was called
    Clipboard { text: &'a str },
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
            sys::PUGL_KEY_PAD_SEPARATOR => Key::NumpadSeparator,
            sys::PUGL_KEY_PAD_CLEAR => Key::NumpadClear,

            _ => match char::from_u32(raw) {
                Some(char) => Key::Char(char),
                None => Key::None,
            },
        }
    }
}

impl<'a, B: Backend> Event<'a, B> {
    pub(crate) unsafe fn process(
        view: *mut sys::PuglView,
        event: *const sys::PuglEvent,
    ) -> Option<Self> {
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
                        from_utf8(&bytes[..len]).ok()?
                    },
                },
                sys::PUGL_POINTER_IN => Event::PointerIn {
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
                sys::PUGL_POINTER_OUT => Event::PointerOut {
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

                sys::PUGL_DATA_OFFER => {
                    let num_types = sys::puglGetNumClipboardTypes(view);
                    for i in 0..num_types {
                        let type_ = sys::puglGetClipboardType(view, i);
                        if CStr::from_ptr(type_).to_str() == Ok("text/plain") {
                            sys::puglAcceptOffer(view, &(*event).offer, i);
                        }
                    }

                    return None;
                }

                sys::PUGL_DATA => {
                    let type_ = sys::puglGetClipboardType(view, (*event).data.typeIndex);
                    if CStr::from_ptr(type_).to_str() == Ok("text/plain") {
                        let mut len = 0;
                        let data = sys::puglGetClipboard(view, (*event).data.typeIndex, &mut len);
                        if !data.is_null() {
                            let text = from_utf8(from_raw_parts(data as *const u8, len)).ok()?;
                            return Some(Event::Clipboard { text });
                        }
                    }

                    return None;
                }

                _ => return None,
            })
        }
    }
}
