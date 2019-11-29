use afl::fuzz;

use anes::parser::Parser;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut parser = Parser::default();
        parser.advance(data, false);
    });
}
