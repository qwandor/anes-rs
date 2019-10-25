//! A terminal buffer related ANSI escape sequences.

impl_csi_sequence!(
    "Switch to the alternate buffer.",
    SwitchToAlternate,
    "?1049h"
);

impl_csi_sequence!("Switch to the normal buffer.", SwitchToNormal, "?1049l");

/// Scroll up by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct ScrollUp(pub u16);

impl ::std::fmt::Display for ScrollUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}S"), self.0)
    }
}

/// Scroll down by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct ScrollDown(pub u16);

impl ::std::fmt::Display for ScrollDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}T"), self.0)
    }
}

/// Erase part of the line.
#[derive(Copy, Clone, Debug)]
pub enum EraseInLine {
    /// Erase from the cursor position to end of the line.
    ToRight,
    /// Erase from the cursor position to beginning of the line.
    ToLeft,
    /// Erase the whole line.
    All,
}

impl ::std::fmt::Display for EraseInLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EraseInLine::ToRight => write!(f, csi!("K")), // or 0K
            EraseInLine::ToLeft => write!(f, csi!("1K")),
            EraseInLine::All => write!(f, csi!("2K")),
        }
    }
}

/// Erase part of the screen.
#[derive(Copy, Clone, Debug)]
pub enum EraseInDisplay {
    /// Erase from the cursor position to end of the screen.
    Below,
    /// Erase from the cursor position to beginning of the screen.
    Above,
    /// Erase the entire screen.
    All,
    /// Erase the entire screen and all saved lines in the scrollback buffer.
    SavedLines,
}

impl ::std::fmt::Display for EraseInDisplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EraseInDisplay::Below => write!(f, csi!("J")), // or 0J
            EraseInDisplay::Above => write!(f, csi!("1J")),
            EraseInDisplay::All => write!(f, csi!("2J")),
            EraseInDisplay::SavedLines => write!(f, csi!("3J")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_to_alternate() {
        assert_eq!(&format!("{}", SwitchToAlternate), "\x1B[?1049h");
    }

    #[test]
    fn test_switch_to_main() {
        assert_eq!(&format!("{}", SwitchToNormal), "\x1B[?1049l");
    }

    #[test]
    fn test_scroll_up() {
        assert_eq!(&format!("{}", ScrollUp(10)), "\x1B[10S");
    }

    #[test]
    fn test_scroll_down() {
        assert_eq!(&format!("{}", ScrollDown(10)), "\x1B[10T");
    }

    #[test]
    fn test_erase_in_line() {
        assert_eq!(&format!("{}", EraseInLine::ToRight), "\x1B[K");
        assert_eq!(&format!("{}", EraseInLine::ToLeft), "\x1B[1K");
        assert_eq!(&format!("{}", EraseInLine::All), "\x1B[2K");
    }

    #[test]
    fn test_erase_in_display() {
        assert_eq!(&format!("{}", EraseInDisplay::Below), "\x1B[J");
        assert_eq!(&format!("{}", EraseInDisplay::Above), "\x1B[1J");
        assert_eq!(&format!("{}", EraseInDisplay::All), "\x1B[2J");
        assert_eq!(&format!("{}", EraseInDisplay::SavedLines), "\x1B[3J");
    }
}
