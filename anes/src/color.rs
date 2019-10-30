use std::fmt;

/// A color.
///
/// This is not a full ANSI sequence. `Color` must be used with:
///
/// * [`SetBackgroundColor`](struct.SetBackgroundColor.html)
/// * [`SetForegroundColor`](struct.SetForegroundColor.html)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    /// Resets the color to the default one.
    Default,
    /// Black color.
    Black,
    /// Dark red color.
    DarkRed,
    /// Dark green color.
    DarkGreen,
    /// Dark yellow color.
    DarkYellow,
    /// Dark blue color.
    DarkBlue,
    /// Dark magenta color.
    DarkMagenta,
    /// Dark cyan color.
    DarkCyan,
    /// Dark gray color.
    ///
    /// Also knows as light (bright) black.
    DarkGray,
    /// Light (bright) gray color.
    ///
    /// Also known as dark white.
    Gray,
    /// Light (bright) red color.
    Red,
    /// Light (bright) green color.
    Green,
    /// Light (bright) yellow color.
    Yellow,
    /// Light (bright) blue color.
    Blue,
    /// Light (bright) magenta color.
    Magenta,
    /// Light (bright) cyan color.
    Cyan,
    /// White color.
    White,
    /// A color from the predefined set of ANSI colors.
    ///
    /// ```text
    ///    0 - 7:  standard colors (as in ESC [ 30–37 m)
    ///    8- 15:  high intensity colors (as in ESC [ 90–97 m)
    ///   16-231:  6 × 6 × 6 cube (216 colors): 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5)
    ///  232-255:  grayscale from black to white in 24 steps
    /// ```
    ///
    /// See [8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) for more information.
    Ansi(u8),
    /// An RGB color.
    ///
    /// See [24-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit) for more information.
    Rgb(u8, u8, u8),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Color::Default is handled in the SetBackgroundColor & SetForegroundColor
            Color::Default => Ok(()),
            Color::Black => write!(f, "5;0"),
            Color::DarkRed => write!(f, "5;1"),
            Color::DarkGreen => write!(f, "5;2"),
            Color::DarkYellow => write!(f, "5;3"),
            Color::DarkBlue => write!(f, "5;4"),
            Color::DarkMagenta => write!(f, "5;5"),
            Color::DarkCyan => write!(f, "5;6"),
            Color::Gray => write!(f, "5;7"),
            Color::DarkGray => write!(f, "5;8"),
            Color::Red => write!(f, "5;9"),
            Color::Green => write!(f, "5;10"),
            Color::Yellow => write!(f, "5;11"),
            Color::Blue => write!(f, "5;12"),
            Color::Magenta => write!(f, "5;13"),
            Color::Cyan => write!(f, "5;14"),
            Color::White => write!(f, "5;15"),
            Color::Ansi(value) => write!(f, "5;{}", value),
            Color::Rgb(r, g, b) => write!(f, "2;{};{};{}", r, g, b),
        }
    }
}

sequence! {
    /// Sets the foreground color.
    ///
    /// # Examples
    ///
    /// White text:
    ///
    /// ```rust
    /// use anes::{Color, SetForegroundColor};
    ///
    /// let white_fg = SetForegroundColor(Color::White);
    /// assert_eq!(&format!("{}", white_fg), "\x1B[38;5;15m");
    /// ```
    struct SetForegroundColor(Color) =>
    |this, f| match this.0 {
        Color::Default => write!(f, csi!("39m")),
        _ => write!(f, csi!("38;{}m"), this.0),
    }
}

sequence! {
    /// Sets the background color.
    ///
    /// # Examples
    ///
    /// Red background:
    ///
    /// ```rust
    /// use anes::{Color, SetBackgroundColor};
    ///
    /// let red_bg = SetBackgroundColor(Color::Red);
    /// assert_eq!(&format!("{}", red_bg), "\x1B[48;5;9m");
    /// ```
    struct SetBackgroundColor(Color) =>
    |this, f| match this.0 {
        Color::Default => write!(f, csi!("49m")),
        _ => write!(f, csi!("48;{}m"), this.0),
    }
}

#[cfg(test)]
test_sequences!(
    test_set_foreground_color(
        SetForegroundColor(Color::Default) => "\x1B[39m",
        SetForegroundColor(Color::Black) => "\x1B[38;5;0m",
        SetForegroundColor(Color::DarkRed) => "\x1B[38;5;1m",
        SetForegroundColor(Color::DarkGreen) => "\x1B[38;5;2m",
        SetForegroundColor(Color::DarkYellow) => "\x1B[38;5;3m",
        SetForegroundColor(Color::DarkBlue) => "\x1B[38;5;4m",
        SetForegroundColor(Color::DarkMagenta) => "\x1B[38;5;5m",
        SetForegroundColor(Color::DarkCyan) => "\x1B[38;5;6m",
        SetForegroundColor(Color::DarkGray) => "\x1B[38;5;8m",
        SetForegroundColor(Color::Gray) => "\x1B[38;5;7m",
        SetForegroundColor(Color::Red) => "\x1B[38;5;9m",
        SetForegroundColor(Color::Green) => "\x1B[38;5;10m",
        SetForegroundColor(Color::Yellow) => "\x1B[38;5;11m",
        SetForegroundColor(Color::Blue) => "\x1B[38;5;12m",
        SetForegroundColor(Color::Magenta) => "\x1B[38;5;13m",
        SetForegroundColor(Color::Cyan) => "\x1B[38;5;14m",
        SetForegroundColor(Color::White) => "\x1B[38;5;15m",
        SetForegroundColor(Color::Ansi(200)) => "\x1B[38;5;200m",
        SetForegroundColor(Color::Rgb(1, 2, 3)) => "\x1B[38;2;1;2;3m",
    ),
    test_set_background_color(
        SetBackgroundColor(Color::Default) => "\x1B[49m",
        SetBackgroundColor(Color::Black) => "\x1B[48;5;0m",
        SetBackgroundColor(Color::DarkRed) => "\x1B[48;5;1m",
        SetBackgroundColor(Color::DarkGreen) => "\x1B[48;5;2m",
        SetBackgroundColor(Color::DarkYellow) => "\x1B[48;5;3m",
        SetBackgroundColor(Color::DarkBlue) => "\x1B[48;5;4m",
        SetBackgroundColor(Color::DarkMagenta) => "\x1B[48;5;5m",
        SetBackgroundColor(Color::DarkCyan) => "\x1B[48;5;6m",
        SetBackgroundColor(Color::DarkGray) => "\x1B[48;5;8m",
        SetBackgroundColor(Color::Gray) => "\x1B[48;5;7m",
        SetBackgroundColor(Color::Red) => "\x1B[48;5;9m",
        SetBackgroundColor(Color::Green) => "\x1B[48;5;10m",
        SetBackgroundColor(Color::Yellow) => "\x1B[48;5;11m",
        SetBackgroundColor(Color::Blue) => "\x1B[48;5;12m",
        SetBackgroundColor(Color::Magenta) => "\x1B[48;5;13m",
        SetBackgroundColor(Color::Cyan) => "\x1B[48;5;14m",        
        SetBackgroundColor(Color::White) => "\x1B[48;5;15m",
        SetBackgroundColor(Color::Ansi(200)) => "\x1B[48;5;200m",
        SetBackgroundColor(Color::Rgb(1, 2, 3)) => "\x1B[48;2;1;2;3m",
    )
);
