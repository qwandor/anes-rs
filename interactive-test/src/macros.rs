macro_rules! run_tests {
    (
        $dst:expr_2021,
        $(
            $testfn:ident
        ),*
        $(,)?
    ) => {
        $(
            ::anes::queue!(
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
