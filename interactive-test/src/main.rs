use std::io::{self, Write};

use crossterm::{input, RawScreen, Result};

use anes;

#[macro_use]
mod macros;

const MENU: &str = r#"ANES interactive test

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests: 

1. TODO
2. TODO

Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, anes::SwitchBufferToAlternate, anes::HideCursor)?;

    let _raw = RawScreen::into_raw_mode()?;

    loop {
        queue!(w, anes::ClearBuffer::All, anes::MoveCursorTo(1, 1))?;
        for line in MENU.split("\n") {
            queue!(w, line, anes::MoveCursorToNextLine(1))?;
        }
        w.flush()?;

        match input().read_char() {
            Ok('q') => break,
            Ok(_) => {}
            Err(e) => return Err(e),
        };
    }

    execute!(w, anes::ShowCursor, anes::SwitchBufferToNormal)?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stderr = io::stderr();
    run(&mut stderr)
}
