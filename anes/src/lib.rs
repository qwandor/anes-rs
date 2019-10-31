//! # ANSI Escape Sequence
//!
//! The `anes` crate provides ANSI escape sequences you can use to control the terminal
//! cursor (show, hide, ...), colors (foreground, background), display attributes (bold, ...)
//! and many others.
//!
//! Every sequence implements the standard library [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! trait. It means that these sequences can be used in macros like
//! [`format!`](https://doc.rust-lang.org/std/macro.format.html) or
//! [`write!`](https://doc.rust-lang.org/std/macro.write.html).
//!
//! Ask if you need more sequences or use the [`sequence!`](macro.sequence.html) macro to create
//! your own sequences.
//!
//! ## Examples
//!
//! Retrieve the sequence as a `String`:
//!
//! ```rust
//! use anes::SaveCursorPosition;
//!
//! let string = format!("{}", SaveCursorPosition);
//! assert_eq!(&string, "\x1B7");
//! ```
//!
//! Use the sequence on the standard output:
//!
//! ```rust
//! use std::io::{Result, Write};
//!
//! fn main() -> Result<()> {
//!     let mut stdout = std::io::stdout();
//!     write!(stdout, "{}", anes::ResetAttributes)?;
//!     stdout.flush()?;
//!     Ok(())
//! }
//! ```

#![warn(rust_2018_idioms)]
#![deny(unused_imports, unused_must_use)]

pub use self::{
    attribute::{Attribute, ResetAttributes, SetAttribute},
    buffer::{
        ClearBuffer, ClearLine, ScrollBufferDown, ScrollBufferUp, SwitchBufferToAlternate,
        SwitchBufferToNormal,
    },
    color::{Color, SetBackgroundColor, SetForegroundColor},
    cursor::{
        DisableCursorBlinking, EnableCursorBlinking, HideCursor, MoveCursorDown, MoveCursorLeft,
        MoveCursorRight, MoveCursorTo, MoveCursorToColumn, MoveCursorToNextLine,
        MoveCursorToPreviousLine, MoveCursorUp, RestoreCursorPosition, SaveCursorPosition,
        ShowCursor,
    },
    terminal::ResizeTextArea,
};

// Keep it first to load all the macros before other modules.
#[macro_use]
mod macros;

mod attribute;
mod buffer;
mod color;
mod cursor;
mod terminal;
