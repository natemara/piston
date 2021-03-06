#![crate_name = "input"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

#[macro_use]
extern crate bitflags;
extern crate rustc_serialize;
extern crate viewport;

pub use mouse::MouseButton;
pub use keyboard::Key;
pub use joystick::{ JoystickAxisArgs, JoystickButton };

pub mod keyboard;
pub mod mouse;
pub mod joystick;

pub use generic_event::GenericEvent;
pub use update::{ UpdateArgs, UpdateEvent };
pub use render::{ RenderArgs, RenderEvent };
pub use after_render::{ AfterRenderArgs, AfterRenderEvent };
pub use idle::{ IdleArgs, IdleEvent };
pub use event::Event;
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use joystick::{ JoystickAxisEvent };
pub use text::TextEvent;
pub use resize::ResizeEvent;
pub use focus::FocusEvent;
pub use cursor::CursorEvent;

pub mod generic_event;
mod update;
mod render;
mod after_render;
mod idle;
mod event;
mod press;
mod release;
mod text;
mod resize;
mod focus;
mod cursor;

/// Used to identify events arguments provided by traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct EventId(pub &'static str);

const FOCUS: EventId = EventId("piston/focus");
const CURSOR: EventId = EventId("piston/cursor");
const RESIZE: EventId = EventId("piston/resize");
const TEXT: EventId = EventId("piston/text");
const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
const JOYSTICK_AXIS: EventId = EventId("piston/joystick_axis");
const RELEASE: EventId = EventId("piston/release");
const PRESS: EventId = EventId("piston/press");
const IDLE: EventId = EventId("piston/idle");
const AFTER_RENDER: EventId = EventId("piston/after_render");
const RENDER: EventId = EventId("piston/render");
const UPDATE: EventId = EventId("piston/update");

/// Models different kinds of buttons.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Hash, Debug)]
pub enum Button {
    /// A keyboard button.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
    /// A joystick button.
    Joystick(JoystickButton),
}

/// Models different kinds of motion.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Motion {
    /// x and y in window coordinates.
    MouseCursor(f64, f64),
    /// x and y in relative coordinates.
    MouseRelative(f64, f64),
    /// x and y in scroll ticks.
    MouseScroll(f64, f64),
    /// joystick axis move event.
    JoystickAxis(JoystickAxisArgs),
}

/// Models input events.
#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Input {
    /// Pressed a button.
    Press(Button),
    /// Released a button.
    Release(Button),
    /// Moved mouse cursor.
    Move(Motion),
    /// Text (usually from keyboard).
    Text(String),
    /// Window got resized.
    Resize(u32, u32),
    /// Window gained or lost focus.
    Focus(bool),
    /// Window gained or lost cursor.
    Cursor(bool),
}

impl From<Key> for Button {
    fn from(key: Key) -> Self {
        Button::Keyboard(key)
    }
}

impl From<MouseButton> for Button {
    fn from(btn: MouseButton) -> Self {
        Button::Mouse(btn)
    }
}

impl From<JoystickButton> for Button {
    fn from(btn: JoystickButton) -> Self {
        Button::Joystick(btn)
    }
}

impl From<JoystickAxisArgs> for Motion {
    fn from(args: JoystickAxisArgs) -> Self {
        Motion::JoystickAxis(args)
    }
}

impl From<Motion> for Input {
    fn from(motion: Motion) -> Self {
        Input::Move(motion)
    }
}
