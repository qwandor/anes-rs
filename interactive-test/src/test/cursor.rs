use std::io::Write;

use anes;

use crate::Result;

#[allow(clippy::cognitive_complexity)]
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    let (width, height) = crate::buffer_size()?;

    run_tests!(
        w,
        (
            anes::MoveCursorTo(1, 4),
            "MoveCursorUp(3)",
            anes::MoveCursorUp(3),
        ),
        ("MoveCursorDown(1)", anes::MoveCursorDown(1),),
        ("MoveCursorLeft(2)", anes::MoveCursorLeft(2),),
        ("MoveCursorRight(2)", anes::MoveCursorRight(2),),
        (
            anes::MoveCursorTo(1, 4),
            "MoveCursorToPreviousLine(3)",
            anes::MoveCursorToPreviousLine(3),
        ),
        ("MoveCursorToNextLine(2)", anes::MoveCursorToNextLine(2),),
        ("MoveCursorToColumn(25) [ ]", anes::MoveCursorToColumn(25),),
        ("ShowCursor", anes::ShowCursor,),
        ("HideCursor", anes::HideCursor,),
        (
            "MoveCursorTo(",
            width,
            ",",
            height,
            ")",
            anes::MoveCursorTo(width, height),
        ),
        (
            anes::SaveCursorPosition,
            anes::MoveCursorTo(10, 10),
            "RestoreCursorPosition (top/left)",
            anes::RestoreCursorPosition,
        ),
    );
    Ok(())
}
