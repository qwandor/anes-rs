use std::collections::VecDeque;

use engine::{Engine, Provide};
use types::Sequence;

mod engine;
mod parsers;
pub(crate) mod types;

#[derive(Default)]
pub struct Parser {
    engine: Engine,
    provider: SequenceProvider,
}

impl Parser {
    pub fn advance_with_slice(&mut self, slice: &[u8], more: bool) {
        for i in 0..slice.len() {
            self.advance(slice[i], i < slice.len() - 1 || more);
        }
    }

    pub fn advance(&mut self, byte: u8, more: bool) {
        self.engine.advance(&mut self.provider, byte, more);
    }
}

impl Iterator for Parser {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        self.provider.next()
    }
}

#[derive(Default)]
struct SequenceProvider {
    esc_o: bool,
    seqs: VecDeque<Sequence>,
}

impl Iterator for SequenceProvider {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        self.seqs.pop_front()
    }
}

impl Provide for SequenceProvider {
    fn provide_char(&mut self, ch: char) {
        //        eprintln!("dispatch_char: {}", ch);

        if let Some(seq) = parsers::parse_char(ch, self.esc_o) {
            self.seqs.push_back(seq);
        }
        self.esc_o = false;
    }

    fn provide_esc_sequence(&mut self, ch: char) {
        if ch == 'O' {
            // Exception
            //
            // Esc O - dispatched as an escape sequence followed by single character (P-S) representing
            // F1-F4 keys. We store Esc O flag only which is then used in the dispatch_char method.
            self.esc_o = true;
        } else {
            self.esc_o = false;
            if let Some(seq) = parsers::parse_esc_sequence(ch) {
                self.seqs.push_back(seq);
            }
        }
    }

    fn provide_csi_sequence(&mut self, parameters: &[u64], ignored_count: usize, ch: char) {
        if let Some(seq) = parsers::parse_csi_sequence(parameters, ignored_count, ch) {
            self.seqs.push_back(seq);
        }

        self.esc_o = false;
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn dispatch_char() {
        let mut parser = Parser::default();
        parser.advance(b'a', false);
        assert!(parser.next().is_some());
    }

    #[test]
    fn dispatch_esc_sequence() {
        let mut parser = Parser::default();
        parser.advance(b'\x1B', true);
        assert!(parser.next().is_none());
        parser.advance(b'a', false);
        assert!(parser.next().is_some());
    }

    #[test]
    fn does_not_dispatch_esc_sequence_with_upper_case_o() {
        let mut parser = Parser::default();
        parser.advance(b'\x1B', true);
        assert!(parser.next().is_none());
        parser.advance(b'O', true);
        assert!(parser.next().is_none());
    }

    #[test]
    fn dispatch_esc_with_upper_case_o_followed_by_char_as_single_sequence() {
        let mut parser = Parser::default();
        parser.advance(b'\x1B', true);
        assert!(parser.next().is_none());
        parser.advance(b'O', true);
        assert!(parser.next().is_none());
        parser.advance(b'P', false);
        assert!(parser.next().is_some());
        assert!(parser.next().is_none());
    }

    #[test]
    fn dispatch_csi_sequence() {
        let mut parser = Parser::default();
        parser.advance(b'\x1B', true);
        assert!(parser.next().is_none());
        parser.advance(b'[', true);
        assert!(parser.next().is_none());
        parser.advance(b'D', false);
        assert!(parser.next().is_some());
    }
}
