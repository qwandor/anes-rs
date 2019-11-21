use std::io::Write;

use anes::execute;

use crate::Result;

fn test_move_cursor_up<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        w,
        anes::MoveCursorTo(1, 4),
        "MoveCursorUp(3)",
        anes::MoveCursorUp(3),
    )?;
    Ok(())
}

fn test_move_cursor_down<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "MoveCursorDown(1)", anes::MoveCursorDown(1),)?;
    Ok(())
}

fn test_move_cursor_left<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "MoveCursorLeft(2)", anes::MoveCursorLeft(2),)?;
    Ok(())
}

fn test_move_cursor_right<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "MoveCursorRight(2)", anes::MoveCursorRight(2),)?;
    Ok(())
}

fn test_move_cursor_to_previous_line<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        w,
        anes::MoveCursorTo(1, 4),
        "MoveCursorToPreviousLine(3)",
        anes::MoveCursorToPreviousLine(3),
    )?;
    Ok(())
}

fn test_move_cursor_to_next_line<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "MoveCursorToNextLine(2)", anes::MoveCursorToNextLine(2),)?;
    Ok(())
}

fn test_move_cursor_to_column<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        w,
        "MoveCursorToColumn(25) [ ]",
        anes::MoveCursorToColumn(25),
    )?;
    Ok(())
}

fn test_hide_cursor<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "HideCursor", anes::HideCursor,)?;
    Ok(())
}

fn test_show_cursor<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "ShowCursor", anes::ShowCursor,)?;
    Ok(())
}

fn test_enable_cursor_blinking<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "EnableCursorBlinking", anes::EnableCursorBlinking,)?;
    Ok(())
}

fn test_disable_cursor_blinking<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, "DisableCursorBlinking", anes::DisableCursorBlinking,)?;
    Ok(())
}

fn test_move_cursor_to<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    let (width, height) = crate::buffer_size()?;

    execute!(
        w,
        "MoveCursorTo(",
        width,
        ",",
        height,
        ")",
        anes::MoveCursorTo(width, height),
    )?;
    Ok(())
}

fn test_save_restore_cursor_position<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        w,
        anes::SaveCursorPosition,
        anes::MoveCursorTo(10, 10),
        "RestoreCursorPosition (top/left)",
        anes::RestoreCursorPosition,
    )?;
    Ok(())
}

#[allow(clippy::cognitive_complexity)]
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(
        w,
        test_hide_cursor,
        test_show_cursor,
        test_enable_cursor_blinking,
        test_disable_cursor_blinking,
        test_move_cursor_left,
        test_move_cursor_right,
        test_move_cursor_up,
        test_move_cursor_down,
        test_move_cursor_to,
        test_move_cursor_to_next_line,
        test_move_cursor_to_previous_line,
        test_move_cursor_to_column,
        test_save_restore_cursor_position,
    );
    Ok(())
}
