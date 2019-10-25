//! A terminal cursor related ANSI escape sequences.

impl_esc_sequence!("Save the cursor position.", SavePosition, "7");

impl_esc_sequence!("Restore the cursor position.", RestorePosition, "8");

impl_csi_sequence!("Hide the cursor.", Hide, "?25l");

impl_csi_sequence!("Show the cursor.", Show, "?25h");

impl_csi_sequence!("Enable the cursor blinking.", EnableBlinking, "?12h");

impl_csi_sequence!("Disable the cursor blinking.", DisableBlinking, "?12l");

/// Move the cursor to the given location (column, row).
///
/// # Notes
///
/// Top/left cell is represented as `0, 0`.
#[derive(Copy, Clone, Debug)]
pub struct MoveTo(pub u16, pub u16);

impl ::std::fmt::Display for MoveTo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{};{}H"), self.0 + 1, self.1 + 1)
    }
}

/// Move up the cursor by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct MoveUp(pub u16);

impl ::std::fmt::Display for MoveUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}A"), self.0)
    }
}

/// Move down the cursor by the given number of rows.
#[derive(Copy, Clone, Debug)]
pub struct MoveDown(pub u16);

impl ::std::fmt::Display for MoveDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}B"), self.0)
    }
}

/// Move right the cursor by the given number of columns.
#[derive(Copy, Clone, Debug)]
pub struct MoveRight(pub u16);

impl ::std::fmt::Display for MoveRight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}C"), self.0)
    }
}

/// Move left the cursor by the given number of columns.
#[derive(Copy, Clone, Debug)]
pub struct MoveLeft(pub u16);

impl ::std::fmt::Display for MoveLeft {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}D"), self.0)
    }
}

/// Move the cursor to beginning of line the given number of lines down.
#[derive(Copy, Clone, Debug)]
pub struct MoveToNextLine(pub u16);

impl ::std::fmt::Display for MoveToNextLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}E"), self.0)
    }
}

/// Move the cursor to beginning of line the given number of lines up.
#[derive(Copy, Clone, Debug)]
pub struct MoveToPreviousLine(pub u16);

impl ::std::fmt::Display for MoveToPreviousLine {
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
pub struct MoveToColumn(pub u16);

impl ::std::fmt::Display for MoveToColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}G"), self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_position() {
        assert_eq!(&format!("{}", SavePosition), "\x1B7");
    }

    #[test]
    fn test_restore_position() {
        assert_eq!(&format!("{}", RestorePosition), "\x1B8");
    }

    #[test]
    fn test_hide() {
        assert_eq!(&format!("{}", Hide), "\x1B[?25l");
    }

    #[test]
    fn test_show() {
        assert_eq!(&format!("{}", Show), "\x1B[?25h");
    }

    #[test]
    fn test_disable_blinking() {
        assert_eq!(&format!("{}", DisableBlinking), "\x1B[?12l");
    }

    #[test]
    fn test_enable_blinking() {
        assert_eq!(&format!("{}", EnableBlinking), "\x1B[?12h");
    }

    #[test]
    fn test_move_up() {
        assert_eq!(&format!("{}", MoveUp(10)), "\x1B[10A");
    }

    #[test]
    fn test_move_down() {
        assert_eq!(&format!("{}", MoveDown(10)), "\x1B[10B");
    }

    #[test]
    fn test_move_right() {
        assert_eq!(&format!("{}", MoveRight(10)), "\x1B[10C");
    }

    #[test]
    fn test_move_left() {
        assert_eq!(&format!("{}", MoveLeft(10)), "\x1B[10D");
    }

    #[test]
    fn test_move_to() {
        assert_eq!(&format!("{}", MoveTo(5, 10)), "\x1B[6;11H");
    }

    #[test]
    fn test_move_to_next_line() {
        assert_eq!(&format!("{}", MoveToNextLine(5)), "\x1B[5E");
    }

    #[test]
    fn test_move_to_previous_line() {
        assert_eq!(&format!("{}", MoveToPreviousLine(5)), "\x1B[5F");
    }

    #[test]
    fn test_move_to_column() {
        assert_eq!(&format!("{}", MoveToColumn(0)), "\x1B[1G");
    }
}
