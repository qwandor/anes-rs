//! An example how to retrieve the ANSI escape sequence as a `String`.
use anes::cursor::SavePosition;

fn main() {
    let string = format!("{}", SavePosition);
    assert_eq!(&string, "\x1B7");
}
