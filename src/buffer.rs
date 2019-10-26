//! A terminal buffer related ANSI escape sequences.

impl_csi_sequence!(
    "Switch to the alternate buffer.",
    SwitchBufferToAlternate,
    "?1049h"
);

impl_csi_sequence!(
    "Switch to the normal buffer.",
    SwitchBufferToNormal,
    "?1049l"
);

/// Scroll up by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct ScrollBufferUp(pub u16);

impl ::std::fmt::Display for ScrollBufferUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}S"), self.0)
    }
}

/// Scroll down by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct ScrollBufferDown(pub u16);

impl ::std::fmt::Display for ScrollBufferDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}T"), self.0)
    }
}

/// Erase part of the line.
#[derive(Copy, Clone, Debug)]
pub enum ClearLine {
    /// Erase from the cursor position to end of the line.
    Right,
    /// Erase from the cursor position to beginning of the line.
    Left,
    /// Erase the whole line.
    All,
}

impl ::std::fmt::Display for ClearLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClearLine::Right => write!(f, csi!("K")), // or 0K
            ClearLine::Left => write!(f, csi!("1K")),
            ClearLine::All => write!(f, csi!("2K")),
        }
    }
}

/// Erase part of the screen.
#[derive(Copy, Clone, Debug)]
pub enum ClearBuffer {
    /// Erase from the cursor position to end of the screen.
    Below,
    /// Erase from the cursor position to beginning of the screen.
    Above,
    /// Erase the entire screen.
    All,
    /// Erase the entire screen and all saved lines in the scrollback buffer.
    SavedLines,
}

impl ::std::fmt::Display for ClearBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClearBuffer::Below => write!(f, csi!("J")), // or 0J
            ClearBuffer::Above => write!(f, csi!("1J")),
            ClearBuffer::All => write!(f, csi!("2J")),
            ClearBuffer::SavedLines => write!(f, csi!("3J")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_buffer_to_alternate() {
        assert_eq!(&format!("{}", SwitchBufferToAlternate), "\x1B[?1049h");
    }

    #[test]
    fn test_switch_buffer_to_main() {
        assert_eq!(&format!("{}", SwitchBufferToNormal), "\x1B[?1049l");
    }

    #[test]
    fn test_scroll_buffer_up() {
        assert_eq!(&format!("{}", ScrollBufferUp(10)), "\x1B[10S");
    }

    #[test]
    fn test_scroll_buffer_down() {
        assert_eq!(&format!("{}", ScrollBufferDown(10)), "\x1B[10T");
    }

    #[test]
    fn test_clear_line() {
        assert_eq!(&format!("{}", ClearLine::Right), "\x1B[K");
        assert_eq!(&format!("{}", ClearLine::Left), "\x1B[1K");
        assert_eq!(&format!("{}", ClearLine::All), "\x1B[2K");
    }

    #[test]
    fn test_clear_buffer() {
        assert_eq!(&format!("{}", ClearBuffer::Below), "\x1B[J");
        assert_eq!(&format!("{}", ClearBuffer::Above), "\x1B[1J");
        assert_eq!(&format!("{}", ClearBuffer::All), "\x1B[2J");
        assert_eq!(&format!("{}", ClearBuffer::SavedLines), "\x1B[3J");
    }
}
