use std::io::Write;

use crate::Result;

const ALL_COLORS: [anes::Color; 21] = [
    anes::Color::Black,
    anes::Color::DarkGray,
    anes::Color::Gray,
    anes::Color::White,
    anes::Color::DarkRed,
    anes::Color::Red,
    anes::Color::DarkGreen,
    anes::Color::Green,
    anes::Color::DarkYellow,
    anes::Color::Yellow,
    anes::Color::DarkBlue,
    anes::Color::Blue,
    anes::Color::DarkMagenta,
    anes::Color::Magenta,
    anes::Color::DarkCyan,
    anes::Color::Cyan,
    anes::Color::Ansi(0),
    anes::Color::Ansi(15),
    anes::Color::Rgb(255, 0, 0),
    anes::Color::Rgb(0, 255, 0),
    anes::Color::Rgb(0, 0, 255),
];

fn test_set_foreground_color<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(
        w,
        "All foreground colors on the black & white background.",
        anes::MoveCursorToNextLine(2),
    )?;

    for color in &ALL_COLORS {
        let set_fg_color = anes::SetForegroundColor(*color);

        queue!(
            w,
            set_fg_color,
            anes::SetBackgroundColor(anes::Color::Black),
            format!(
                "{:>width$} ",
                format!("{:?} ████████████", color),
                width = 35
            ),
            anes::SetBackgroundColor(anes::Color::White),
            format!(
                "{:>width$}",
                format!("{:?} ████████████", color),
                width = 35
            ),
            anes::MoveCursorToNextLine(1),
        )?;
    }

    w.flush()?;

    Ok(())
}

fn test_set_background_color<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(
        w,
        "All background colors with black & white foreground.",
        anes::MoveCursorToNextLine(2),
    )?;

    for color in &ALL_COLORS {
        let set_bg_color = anes::SetBackgroundColor(*color);

        queue!(
            w,
            set_bg_color,
            anes::SetForegroundColor(anes::Color::Black),
            format!(
                "{:>width$} ",
                format!("{:?} ▒▒▒▒▒▒▒▒▒▒▒▒", color),
                width = 35
            ),
            anes::SetForegroundColor(anes::Color::White),
            format!(
                "{:>width$}",
                format!("{:?} ▒▒▒▒▒▒▒▒▒▒▒▒", color),
                width = 35
            ),
            anes::MoveCursorToNextLine(1),
        )?;
    }

    w.flush()?;

    Ok(())
}

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(w, test_set_foreground_color, test_set_background_color,);
    Ok(())
}
