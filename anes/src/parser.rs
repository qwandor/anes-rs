use std::collections::VecDeque;

use engine::{Engine, Perform};
use types::Sequence;

mod engine;
mod parsers;
pub(crate) mod types;

#[derive(Default)]
pub struct Parser {
    engine: Engine,
    performer: Performer,
}

impl Parser {
    pub fn advance(&mut self, byte: u8, more: bool) {
        self.engine.advance(&mut self.performer, byte, more);
    }
}

impl Iterator for Parser {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        self.performer.next()
    }
}

#[derive(Default)]
struct Performer {
    esc_o: bool,
    seqs: VecDeque<Sequence>,
}

impl Iterator for Performer {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        self.seqs.pop_front()
    }
}

impl Perform for Performer {
    fn dispatch_char(&mut self, ch: char) {
        //        eprintln!("dispatch_char: {}", ch);

        if let Some(seq) = parsers::parse_char(ch, self.esc_o) {
            self.seqs.push_back(seq);
        }
        self.esc_o = false;
    }

    fn dispatch_esc(&mut self, intermediates: &[u8], ignored_intermediates_count: usize, ch: char) {
        //        eprintln!(
        //            "dispatch_esc: {:?} {} {}",
        //            intermediates, ignored_intermediates_count, ch
        //        );

        if ch == 'O' {
            // Exception
            //
            // Esc O - dispatched as an escape sequence followed by single character (P-S) representing
            // F1-F4 keys. We store Esc O flag only which is then used in the dispatch_char method.
            self.esc_o = true;
        } else {
            self.esc_o = false;
            if let Some(seq) = parsers::parse_esc(intermediates, ignored_intermediates_count, ch) {
                self.seqs.push_back(seq);
            }
        }
    }

    fn dispatch_csi(
        &mut self,
        parameters: &[u64],
        ignored_parameters_count: usize,
        intermediates: &[u8],
        ignored_intermediates_count: usize,
        ch: char,
    ) {
        //        eprintln!(
        //            "dispatch_csi: {:?} {} {:?} {} {}",
        //            parameters, ignored_parameters_count, intermediates, ignored_intermediates_count, ch
        //        );

        if let Some(seq) = parsers::parse_csi(
            parameters,
            ignored_parameters_count,
            intermediates,
            ignored_intermediates_count,
            ch,
        ) {
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
