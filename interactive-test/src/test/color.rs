use std::io::Write;

use anes::queue;

use crate::Result;

const COLORS: [anes::Color; 21] = [
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
        "Foreground colors on the black & white background",
        anes::MoveCursorToNextLine(2),
    )?;

    for color in &COLORS {
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
        "Background colors with black & white foreground",
        anes::MoveCursorToNextLine(2),
    )?;

    for color in &COLORS {
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

fn test_color_values_matrix_16x16<W, F>(w: &mut W, title: &str, color: F) -> Result<()>
where
    W: Write,
    F: Fn(u16, u16) -> anes::Color,
{
    queue!(w, title)?;

    for idx in 0..=15 {
        queue!(
            w,
            anes::MoveCursorTo(1, idx + 4),
            format!("{:>width$}", idx, width = 2)
        )?;
        queue!(
            w,
            anes::MoveCursorTo(idx * 3 + 3, 3),
            format!("{:>width$}", idx, width = 3)
        )?;
    }

    for row in 0..=15u16 {
        queue!(w, anes::MoveCursorTo(4, row + 4))?;
        for col in 0..=15u16 {
            queue!(w, anes::SetForegroundColor(color(col, row)), "███")?;
        }
        queue!(
            w,
            anes::SetForegroundColor(anes::Color::Default),
            " ",
            format!("{:>width$}", row * 16, width = 3),
            " ..= ",
            format!("{:>width$}", row * 16 + 15, width = 3),
        )?;
    }

    w.flush()?;

    Ok(())
}

fn test_color_ansi_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Ansi values", |col, row| {
        anes::Color::Ansi((row * 16 + col) as u8)
    })
}

fn test_rgb_red_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb red values", |col, row| {
        anes::Color::Rgb((row * 16 + col) as u8, 0, 0)
    })
}

fn test_rgb_green_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb green values", |col, row| {
        anes::Color::Rgb(0, (row * 16 + col) as u8, 0)
    })
}

fn test_rgb_blue_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb blue values", |col, row| {
        anes::Color::Rgb(0, 0, (row * 16 + col) as u8)
    })
}

#[allow(clippy::cognitive_complexity)]
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(
        w,
        test_set_foreground_color,
        test_set_background_color,
        test_color_ansi_values,
        test_rgb_red_values,
        test_rgb_green_values,
        test_rgb_blue_values,
    );
    Ok(())
}
