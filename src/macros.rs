/// Creates a control sequence.
///
/// This macro prepends provided sequence with the control sequence introducer (`ESC [`).
///
/// # Examples
///
/// ```
/// use anes::csi;
///
/// let switch_to_alternate_screen = csi!("?1049h");
/// assert_eq!("\x1B[?1049h", switch_to_alternate_screen);
/// ```
#[macro_export]
macro_rules! csi {
    ($($arg:expr),*) => { concat!("\x1B[", $($arg),*) };
}

/// Creates an escape sequence.
///
/// This macro prepends provided sequence with the `ESC` character.
///
/// # Examples
///
/// ```
/// use anes::esc;
///
/// let save_cursor_position = esc!("7");
/// assert_eq!("\x1B7", save_cursor_position);
/// ```
#[macro_export]
macro_rules! esc {
    ($($arg:expr),*) => { concat!("\x1B", $($arg),*) };
}

macro_rules! impl_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug)]
        pub struct $name;

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, $value)
            }
        }
    }
}

macro_rules! impl_csi_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        impl_sequence!($doc, $name, csi!($value));
    };
}

macro_rules! impl_esc_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        impl_sequence!($doc, $name, esc!($value));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_csi() {
        assert_eq!(csi!("foo"), "\x1B[foo");
    }

    #[test]
    fn test_esc() {
        assert_eq!(esc!("bar"), "\x1Bbar");
    }

    #[test]
    fn test_impl_csi_sequence() {
        impl_csi_sequence!("", TestSeq, "foo");
        assert_eq!(&format!("{}", TestSeq), "\x1B[foo");
    }

    #[test]
    fn test_impl_esc_sequence() {
        impl_esc_sequence!("", TestSeq, "bar");
        assert_eq!(&format!("{}", TestSeq), "\x1Bbar");
    }
}
