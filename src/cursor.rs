//! A terminal cursor related ANSI escape sequences.

sequence!(
    /// Saves the cursor position.    
    struct SaveCursorPosition => esc!("7")    
);

sequence!(
    /// Restores the cursor position.
    struct RestoreCursorPosition => esc!("8")
);

sequence!(
    /// Hides the cursor.
    struct HideCursor => csi!("?25l")
);

sequence!(
    /// Shows the cursor.
    struct ShowCursor => csi!("?25h")
);

sequence!(
    /// Enables the cursor blinking.
    struct EnableCursorBlinking => csi!("?12h")
);

sequence!(
    /// Disables the cursor blinking.
    struct DisableCursorBlinking => csi!("?12l")
);

sequence!(
    /// Moves the cursor to the given location (column, row).
    ///
    /// # Notes
    ///
    /// Top/left cell is represented as `1, 1`.
    struct MoveCursorTo(u16, u16) =>
    |this, f| write!(f, csi!("{};{}H"), this.0, this.1)
);

sequence!(
    /// Moves the cursor up by the given number of rows.
    struct MoveCursorUp(u16) =>
    |this, f| write!(f, csi!("{}A"), this.0)
);

sequence!(
    /// Moves the cursor down by the given number of rows.
    struct MoveCursorDown(u16) =>
    |this, f| write!(f, csi!("{}B"), this.0)
);

sequence!(
    /// Moves the cursor right by the given number of columns.
    struct MoveCursorRight(u16) =>
    |this, f| write!(f, csi!("{}C"), this.0)
);

sequence!(
    /// Moves the cursor left by the given number of columns.
    struct MoveCursorLeft(u16) =>
    |this, f| write!(f, csi!("{}D"), this.0)
);

sequence!(
    /// Moves the cursor to beginning of line the given number of lines down.
    struct MoveCursorToNextLine(u16) =>
    |this, f| write!(f, csi!("{}E"), this.0)
);

sequence!(
    /// Moves the cursor to beginning of line the given number of lines up.
    struct MoveCursorToPreviousLine(u16) =>
    |this, f| write!(f, csi!("{}F"), this.0)
);

sequence!(
    /// Moves the cursor to the given column.
    ///
    /// # Notes
    ///
    /// Beginning of the line (left cell) is represented as `1`.
    struct MoveCursorToColumn(u16) =>
    |this, f| write!(f, csi!("{}G"), this.0)
);

#[cfg(test)]
test_sequences!(
    test_save_cursor_position(
        SaveCursorPosition => "\x1B7",
    ),
    test_restore_cursor_position(
        RestoreCursorPosition => "\x1B8",
    ),
    test_hide_cursor(
        HideCursor => "\x1B[?25l",
    ),
    test_show_cursor(
        ShowCursor => "\x1B[?25h",
    ),
    test_disable_cursor_blinking(
        DisableCursorBlinking => "\x1B[?12l",
    ),
    test_enable_cursor_blinking(
        EnableCursorBlinking => "\x1B[?12h",
    ),
    test_move_cursor_up(
        MoveCursorUp(10) => "\x1B[10A",
    ),
    test_move_cursor_down(
        MoveCursorDown(10) => "\x1B[10B",
    ),
    test_move_cursor_right(
        MoveCursorRight(10) => "\x1B[10C",
    ),
    test_move_cursor_left(
        MoveCursorLeft(10) => "\x1B[10D",
    ),
    test_move_cursor_to(
        MoveCursorTo(5, 10) => "\x1B[5;10H",
    ),
    test_move_cursor_to_next_line(
        MoveCursorToNextLine(5) => "\x1B[5E",
    ),
    test_move_cursor_to_previous_line(
        MoveCursorToPreviousLine(5) => "\x1B[5F",
    ),
    test_move_cursor_to_column(
        MoveCursorToColumn(1) => "\x1B[1G",
    ),
);
