//! # ANSI Escape Sequence
//!
//! Every sequence implements the standard library [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! trait. It means that you can use macros like `format!`, `write!` to work with them.
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
//!     write!(stdout, "{}", anes::SaveCursorPosition)?;
//!     write!(stdout, "{}", anes::RestoreCursorPosition)?;
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
