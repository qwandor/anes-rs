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
}
