/// Creates a control sequence.
///
/// This macro prepends provided sequence with the control sequence introducer `ESC [` (`\x1B[`).
///
/// # Examples
///
/// ```
/// use anes::csi;
///
/// let switch_to_alternate_screen = csi!("?1049h");
/// assert_eq!(switch_to_alternate_screen, "\x1B[?1049h");
/// ```
#[macro_export]
macro_rules! csi {
    ($($arg:expr),*) => { concat!("\x1B[", $($arg),*) };
}

/// Creates an escape sequence.
///
/// This macro prepends provided sequence with the `ESC` (`\x1B`) character.
///
/// # Examples
///
/// ```
/// use anes::esc;
///
/// let save_cursor_position = esc!("7");
/// assert_eq!(save_cursor_position, "\x1B7");
/// ```
#[macro_export]
macro_rules! esc {
    ($($arg:expr),*) => { concat!("\x1B", $($arg),*) };
}

/// Creates an ANSI sequence.
///
/// You can use this macro to create your own ANSI sequence. All `anes` sequences are
/// created with this macro.
///
/// # Examples
///
/// An unit struct:
///
/// ```
/// use anes::{esc, sequence};
///
/// sequence!(
///   /// Saves the cursor position.    
///   struct SaveCursorPosition => esc!("7")    
/// );
///
/// assert_eq!(&format!("{}", SaveCursorPosition), "\x1B7");
/// ```
///
/// An enum:
///
/// ```
/// use anes::{csi, sequence};
///
/// sequence!(
///     /// Erases part of the line.
///     enum ClearBuffer {
///         /// Erase from the cursor position to end of the screen.
///         Below => csi!("J"),
///         /// Erase from the cursor position to beginning of the screen.
///         Above => csi!("1J"),
///         /// Erase the entire screen.
///         All => csi!("2J"),
///         /// Erase the entire screen and all saved lines in the scrollback buffer.
///         SavedLines => csi!("3J"),
///     }
/// );
///
/// assert_eq!(&format!("{}", ClearBuffer::Below), "\x1B[J");
/// assert_eq!(&format!("{}", ClearBuffer::Above), "\x1B[1J");
/// assert_eq!(&format!("{}", ClearBuffer::All), "\x1B[2J");
/// assert_eq!(&format!("{}", ClearBuffer::SavedLines), "\x1B[3J");
/// ```
///
/// A struct:
///
/// ```
/// use anes::{csi, sequence};
///
/// sequence!(
///     /// Moves the cursor to the given location (column, row).
///     ///
///     /// # Notes
///     ///
///     /// Top/left cell is represented as `1, 1`.
///     struct MoveCursorTo(u16, u16) =>
///     |this, f| write!(f, csi!("{};{}H"), this.0, this.1)
/// );
///
/// assert_eq!(&format!("{}", MoveCursorTo(10, 5)), "\x1B[10;5H");
/// ```
#[macro_export]
macro_rules! sequence {
    // Static unit struct
    (
        $(#[$meta:meta])*
        struct $name:ident => $value:expr
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, $value)
            }
        }
    };
    // Static enum
    (
        $(#[$meta:meta])*
        enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $variant_value:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        $name::$variant => $variant_value,
                    )*
                })
            }
        }
    };
    // Dynamic struct
    (
        $(#[$meta:meta])*
        struct $type:ident(
            $($fields:ty),*
            $(,)?
        )
        =>
        $write:expr
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $type($(pub $fields),*);

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let write: &dyn Fn(&Self, &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result =
                    &$write;
                write(self, f)
            }
        }
    };
}

#[cfg(test)]
macro_rules! test_sequences {
    (
        $(
            $name:ident(
                $($left:expr => $right:expr),*
                $(,)?
            )
        ),*
        $(,)?
    ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!(&format!("{}", $left), $right);
                    )*
                }
            )*
        }
    }
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
    fn test_static_struct_sequence() {
        sequence!(
            struct TestSeq => csi!("foo")
        );

        assert_eq!(&format!("{}", TestSeq), "\x1B[foo");
    }

    #[test]
    fn test_static_enum_sequence() {
        sequence!(
            enum TestSeq {
                Foo => csi!("foo"),
                Bar => esc!("bar"),
            }
        );

        assert_eq!(&format!("{}", TestSeq::Foo), "\x1B[foo");
        assert_eq!(&format!("{}", TestSeq::Bar), "\x1Bbar");
    }

    #[test]
    fn test_dynamic_struct_sequence() {
        sequence!(
            struct TestSeq(u16) =>
            |this, f| write!(f, csi!("foo{}bar"), this.0)
        );

        assert_eq!(&format!("{}", TestSeq(10)), "\x1B[foo10bar");
    }
}
