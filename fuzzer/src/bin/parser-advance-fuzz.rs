use afl::fuzz;
use anes::Parser;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut parser = Parser::default();
        parser.advance_with_slice(data, false);
    });
}
