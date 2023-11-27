use anes::{execute, queue};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::enable_raw_mode,
};
use std::io::{self, Result, Write};

#[macro_use]
mod macros;
mod test;

const MENU: &str = r#"ANES interactive test

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests: 

1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)

Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, anes::SwitchBufferToAlternate)?;

    enable_raw_mode()?;

    loop {
        queue!(
            w,
            anes::ResetAttributes,
            anes::ClearBuffer::All,
            anes::HideCursor,
            anes::MoveCursorTo(1, 1),
        )?;
        for line in MENU.split('\n') {
            queue!(w, line, anes::MoveCursorToNextLine(1))?;
        }
        w.flush()?;

        match read_char()? {
            '1' => test::cursor::run(w)?,
            '2' => test::color::run(w)?,
            '3' => test::attribute::run(w)?,
            'q' => break,
            _ => {}
        };
    }

    execute!(
        w,
        anes::ResetAttributes,
        anes::ShowCursor,
        anes::SwitchBufferToNormal
    )?;
    Ok(())
}

pub fn read_char() -> Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event::read()?
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> Result<(u16, u16)> {
    crossterm::terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stderr();
    run(&mut stderr)
}
