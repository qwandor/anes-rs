macro_rules! queue {
    ($dst:expr, $($command:expr),* $(,)?) => {{
        let mut error = None;

        $(
            if let Err(e) = write!($dst, "{}", $command) {
                error = Some(e);
            }
        )*

        if let Some(error) = error {
            Err(error)
        } else {
            Ok(())
        }
    }}
}

macro_rules! execute {
    ($dst:expr, $($command:expr),* $(,)?) => {{
        queue!($dst, $($command),*)?;
        $dst.flush().map_err(crossterm::ErrorKind::IoError)
    }}
}

macro_rules! run_tests {
    (
        $dst:expr,
        $(
            $testfn:ident
        ),*
        $(,)?
    ) => {
        $(
            queue!(
                $dst,
                anes::ResetAttributes,
                anes::ClearBuffer::All,
                anes::MoveCursorTo(1, 1),
                anes::ShowCursor,
                anes::EnableCursorBlinking,
            )?;

            $testfn($dst)?;

            match $crate::read_char() {
                Ok('q') => return Ok(()),
                Err(e) => return Err(e),
                _ => {},
            };
        )*
    }
}
