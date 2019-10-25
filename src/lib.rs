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
//! use anes::cursor::SavePosition;
//!
//! let string = format!("{}", SavePosition);
//! assert_eq!(&string, "\x1B7");
//! ```
//!
//! Use the sequence on the standard output:
//!
//! ```rust
//! use std::io::{Result, Write};
//!
//! use anes::cursor;
//!
//! fn main() -> Result<()> {
//!     let mut stdout = std::io::stdout();
//!     write!(stdout, "{}", cursor::SavePosition)?;
//!     write!(stdout, "{}", cursor::RestorePosition)?;
//!     stdout.flush()?;
//!     Ok(())
//! }
//! ```

#![warn(rust_2018_idioms)]
#![deny(unused_imports, unused_must_use)]

// Keep it first to load all the macros before other modules.
#[macro_use]
mod macros;

pub mod buffer;
pub mod cursor;
