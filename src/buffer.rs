//! A terminal buffer related ANSI escape sequences.

sequence!(
    /// Switches to the alternate buffer.
    struct SwitchBufferToAlternate => csi!("?1049h")
);

sequence!(
    /// Switches to the normal buffer.
    struct SwitchBufferToNormal => csi!("?1049l")
);

sequence!(
    /// Scrolls up by the given number of rows.
    struct ScrollBufferUp(u16) =>
    |this, f| write!(f, csi!("{}S"), this.0)
);

sequence!(
    /// Scrolls down by the given number of rows.
    struct ScrollBufferDown(u16) =>
    |this, f| write!(f, csi!("{}T"), this.0)
);

sequence!(
    /// Clears part of the line.
    enum ClearLine {
        /// Clears from the cursor position to end of the line.
        Right => csi!("K"),
        /// Clears from the cursor position to beginning of the line.
        Left => csi!("1K"),
        /// Clears the whole line.
        All => csi!("2K"),
    }
);

sequence!(
    /// Clears part of the buffer.
    enum ClearBuffer {
        /// Clears from the cursor position to end of the screen.
        Below => csi!("J"),
        /// Clears from the cursor position to beginning of the screen.
        Above => csi!("1J"),
        /// Clears the entire buffer.
        All => csi!("2J"),
        /// Clears the entire buffer and all saved lines in the scrollback buffer.
        SavedLines => csi!("3J"),
    }
);

#[cfg(test)]
test_sequences!(
    test_switch_buffer_to_alternate(
        SwitchBufferToAlternate => "\x1B[?1049h",
    ),
    test_switch_buffer_to_main(
        SwitchBufferToNormal => "\x1B[?1049l",
    ),
    test_scroll_buffer_up(
        ScrollBufferUp(10) => "\x1B[10S",
    ),
    test_scroll_buffer_down(
        ScrollBufferDown(10) => "\x1B[10T",
    ),
    test_clear_line(
        ClearLine::Right => "\x1B[K",
        ClearLine::Left => "\x1B[1K",
        ClearLine::All => "\x1B[2K",
    ),
    test_clear_buffer(
        ClearBuffer::Below => "\x1B[J",
        ClearBuffer::Above => "\x1B[1J",
        ClearBuffer::All => "\x1B[2J",
        ClearBuffer::SavedLines => "\x1B[3J",
    ),
);