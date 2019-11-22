#[macro_export]
macro_rules! test_sequence {
    ($bytes:expr, $seq:expr) => {
        let mut parser = ::anes::Parser::default();

        let len = $bytes.len();
        for (i, byte) in $bytes.iter().enumerate() {
            parser.advance(*byte, i < len - 1);
        }

        assert_eq!(parser.next(), Some($seq));
    };
}

#[macro_export]
macro_rules! test_sequences {
    (
        $(
            $bytes:expr, $seq:expr,
        )*
    ) => {
        $(
            test_sequence!($bytes, $seq);
        )*
    };
}

mod cursor;
mod key;
mod mouse;
