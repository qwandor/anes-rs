//! A terminal cursor related ANSI escape sequences.

impl_esc_sequence!("Save the cursor position.", SaveCursorPosition, "7");

impl_esc_sequence!("Restore the cursor position.", RestoreCursorPosition, "8");

impl_csi_sequence!("Hide the cursor.", HideCursor, "?25l");

impl_csi_sequence!("Show the cursor.", ShowCursor, "?25h");

impl_csi_sequence!("Enable the cursor blinking.", EnableCursorBlinking, "?12h");

impl_csi_sequence!(
    "Disable the cursor blinking.",
    DisableCursorBlinking,
    "?12l"
);

/// Move the cursor to the given location (column, row).
///
/// # Notes
///
/// Top/left cell is represented as `1, 1`.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorTo(pub u16, pub u16);

impl ::std::fmt::Display for MoveCursorTo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{};{}H"), self.0, self.1)
    }
}

/// Move up the cursor by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorUp(pub u16);

impl ::std::fmt::Display for MoveCursorUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}A"), self.0)
    }
}

/// Move down the cursor by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorDown(pub u16);

impl ::std::fmt::Display for MoveCursorDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}B"), self.0)
    }
}

/// Move right the cursor by the given number of columns.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorRight(pub u16);

impl ::std::fmt::Display for MoveCursorRight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}C"), self.0)
    }
}

/// Move left the cursor by the given number of columns.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorLeft(pub u16);

impl ::std::fmt::Display for MoveCursorLeft {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}D"), self.0)
    }
}

/// Move the cursor to beginning of line the given number of lines down.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorToNextLine(pub u16);

impl ::std::fmt::Display for MoveCursorToNextLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}E"), self.0)
    }
}

/// Move the cursor to beginning of line the given number of lines up.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorToPreviousLine(pub u16);

impl ::std::fmt::Display for MoveCursorToPreviousLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}F"), self.0)
    }
}

/// Move the cursor to the given column.
///
/// # Notes
///
/// Beginning of the line (left cell) is represented as `0`.
#[derive(Copy, Clone, Debug)]
pub struct MoveCursorToColumn(pub u16);

impl ::std::fmt::Display for MoveCursorToColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}G"), self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_cursor_position() {
        assert_eq!(&format!("{}", SaveCursorPosition), "\x1B7");
    }

    #[test]
    fn test_restore_cursor_position() {
        assert_eq!(&format!("{}", RestoreCursorPosition), "\x1B8");
    }

    #[test]
    fn test_hide_cursor() {
        assert_eq!(&format!("{}", HideCursor), "\x1B[?25l");
    }

    #[test]
    fn test_show_cursor() {
        assert_eq!(&format!("{}", ShowCursor), "\x1B[?25h");
    }

    #[test]
    fn test_disable_cursor_blinking() {
        assert_eq!(&format!("{}", DisableCursorBlinking), "\x1B[?12l");
    }

    #[test]
    fn test_enable_cursor_blinking() {
        assert_eq!(&format!("{}", EnableCursorBlinking), "\x1B[?12h");
    }

    #[test]
    fn test_move_cursor_up() {
        assert_eq!(&format!("{}", MoveCursorUp(10)), "\x1B[10A");
    }

    #[test]
    fn test_move_cursor_down() {
        assert_eq!(&format!("{}", MoveCursorDown(10)), "\x1B[10B");
    }

    #[test]
    fn test_move_cursor_right() {
        assert_eq!(&format!("{}", MoveCursorRight(10)), "\x1B[10C");
    }

    #[test]
    fn test_move_cursor_left() {
        assert_eq!(&format!("{}", MoveCursorLeft(10)), "\x1B[10D");
    }

    #[test]
    fn test_move_cursor_to() {
        assert_eq!(&format!("{}", MoveCursorTo(5, 10)), "\x1B[5;10H");
    }

    #[test]
    fn test_move_cursor_to_next_line() {
        assert_eq!(&format!("{}", MoveCursorToNextLine(5)), "\x1B[5E");
    }

    #[test]
    fn test_move_cursor_to_previous_line() {
        assert_eq!(&format!("{}", MoveCursorToPreviousLine(5)), "\x1B[5F");
    }

    #[test]
    fn test_move_cursor_to_column() {
        assert_eq!(&format!("{}", MoveCursorToColumn(1)), "\x1B[1G");
    }
}
