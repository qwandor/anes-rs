//!
//! TODO - top/left is 0,0
//!

impl_esc_sequence!(
    "A control sequence to save the cursor position.",
    SavePosition,
    "7"
);
impl_esc_sequence!(
    "A control sequence to restore the cursor position.",
    RestorePosition,
    "8"
);

impl_csi_sequence!("A control sequence to hide the cursor.", Hide, "?25l");
impl_csi_sequence!("A control sequence to show the cursor.", Show, "?25h");

impl_csi_sequence!(
    "A control sequence to enable the cursor blinking.",
    EnableBlinking,
    "?12h"
);
impl_csi_sequence!(
    "A control sequence to disable the cursor blinking.",
    DisableBlinking,
    "?12l"
);

/// A control sequence to move the cursor to the given location (column, row).
#[derive(Copy, Clone, Debug)]
pub struct MoveTo(pub u16, pub u16);

impl ::std::fmt::Display for MoveTo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{};{}H"), self.0 + 1, self.1 + 1)
    }
}

/// A control sequence to move the cursor by the given number of rows up.
#[derive(Copy, Clone, Debug)]
pub struct MoveUp(pub u16);

impl ::std::fmt::Display for MoveUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}A"), self.0)
    }
}

/// A control sequence to move the cursor by the given number of rows down.
#[derive(Copy, Clone, Debug)]
pub struct MoveDown(pub u16);

impl ::std::fmt::Display for MoveDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}B"), self.0)
    }
}

/// A control sequence to move the cursor by the given number of columns right.
#[derive(Copy, Clone, Debug)]
pub struct MoveRight(pub u16);

impl ::std::fmt::Display for MoveRight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}C"), self.0)
    }
}

/// A control sequence to move the cursor by the given number of columns right.
#[derive(Copy, Clone, Debug)]
pub struct MoveLeft(pub u16);

impl ::std::fmt::Display for MoveLeft {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("{}D"), self.0)
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
}
