use bitflags::bitflags;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sequence {
    Key(KeyCode, KeyModifiers),
    Mouse(Mouse, KeyModifiers),
    CursorPosition(u16, u16),
}

bitflags! {
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const META = 0b0000_1000;
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Mouse {
    Down(MouseButton, u16, u16),
    Up(MouseButton, u16, u16),
    Drag(MouseButton, u16, u16),
    ScrollDown(u16, u16),
    ScrollUp(u16, u16),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Any,
}
