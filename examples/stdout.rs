/// An example how to use the ANSI escape sequence.
use std::io::{Result, Write};

use anes::cursor;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    write!(stdout, "{}", cursor::SavePosition)?;
    write!(stdout, "{}", cursor::RestorePosition)?;
    stdout.flush()?;
    Ok(())
}
