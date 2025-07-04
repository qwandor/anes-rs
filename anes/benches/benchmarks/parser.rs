use criterion::{Criterion, black_box, criterion_group};

use anes::parser::Parser;

pub fn parser(c: &mut Criterion) {
    const XTERM_MOUSE: &str = "\x1B[<28;20;10;m";

    let mut parser = Parser::default();

    c.bench_function("advance and consume", |b| {
        let input = XTERM_MOUSE.as_bytes();

        b.iter(|| {
            parser.advance(black_box(input), black_box(true));
            for _ in parser.by_ref() {}
        })
    });
}

criterion_group!(benches, parser);
