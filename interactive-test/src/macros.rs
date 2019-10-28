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
        $dst.flush()
    }}
}
