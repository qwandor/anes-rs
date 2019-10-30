use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Default,
    Black,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    DarkGrey,
    Grey,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Color::Default => "",
            Color::Black => "5;0",
            Color::DarkRed => "5;1",
            Color::DarkGreen => "5;2",
            Color::DarkYellow => "5;3",
            Color::DarkBlue => "5;4",
            Color::DarkMagenta => "5;5",
            Color::DarkCyan => "5;6",
            Color::Grey => "5;7",
            Color::DarkGrey => "5;8",
            Color::Red => "5;9",
            Color::Green => "5;10",
            Color::Yellow => "5;11",
            Color::Blue => "5;12",
            Color::Magenta => "5;13",
            Color::Cyan => "5;14",
            Color::White => "5;15",
        };
        write!(f, "{}", code)
    }
}

sequence! {
    struct SetForegroundColor(Color) =>
    |this, f| match this.0 {
        Color::Default => write!(f, csi!("39m")),
        _ => write!(f, csi!("38;{}m"), this.0),
    }
}

sequence! {
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
        SetForegroundColor(Color::DarkGrey) => "\x1B[38;5;8m",
        SetForegroundColor(Color::Grey) => "\x1B[38;5;7m",
        SetForegroundColor(Color::Red) => "\x1B[38;5;9m",
        SetForegroundColor(Color::Green) => "\x1B[38;5;10m",
        SetForegroundColor(Color::Yellow) => "\x1B[38;5;11m",
        SetForegroundColor(Color::Blue) => "\x1B[38;5;12m",
        SetForegroundColor(Color::Magenta) => "\x1B[38;5;13m",
        SetForegroundColor(Color::Cyan) => "\x1B[38;5;14m",
        SetForegroundColor(Color::White) => "\x1B[38;5;15m",
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
        SetBackgroundColor(Color::DarkGrey) => "\x1B[48;5;8m",
        SetBackgroundColor(Color::Grey) => "\x1B[48;5;7m",
        SetBackgroundColor(Color::Red) => "\x1B[48;5;9m",
        SetBackgroundColor(Color::Green) => "\x1B[48;5;10m",
        SetBackgroundColor(Color::Yellow) => "\x1B[48;5;11m",
        SetBackgroundColor(Color::Blue) => "\x1B[48;5;12m",
        SetBackgroundColor(Color::Magenta) => "\x1B[48;5;13m",
        SetBackgroundColor(Color::Cyan) => "\x1B[48;5;14m",        
        SetBackgroundColor(Color::White) => "\x1B[48;5;15m",
    )
);
