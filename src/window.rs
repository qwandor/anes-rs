//! A terminal window related ANSI escape sequences.
//!
/// Resize the text area to the given width and height in characters.
#[derive(Copy, Clone, Debug)]
pub struct ResizeTextArea(pub u16, pub u16);

impl ::std::fmt::Display for ResizeTextArea {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, csi!("8;{};{}t"), self.1, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_text_area() {
        assert_eq!(&format!("{}", ResizeTextArea(80, 25)), "\x1B[8;25;80t");
    }
}
