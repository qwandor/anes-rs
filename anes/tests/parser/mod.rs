#[macro_export]
macro_rules! test_sequence {
    ($bytes:expr_2021, $seq:expr_2021) => {
        let mut parser = ::anes::parser::Parser::default();
        parser.advance($bytes, false);
        assert_eq!(parser.next(), Some($seq));
    };
}

#[macro_export]
macro_rules! test_sequences {
    (
        $(
            $bytes:expr_2021, $seq:expr_2021,
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
