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
    use super::{
        types::{KeyCode, KeyModifiers, Mouse, MouseButton, Sequence},
        Parser,
    };

    macro_rules! test_sequence {
        ($bytes:expr, $seq:expr) => {
            let mut parser = Parser::default();

            let len = $bytes.len();
            for (i, byte) in $bytes.iter().enumerate() {
                parser.advance(*byte, i < len - 1);
            }

            assert_eq!(parser.next(), Some($seq));
        };
    }

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

    #[test]
    fn esc_char() {
        test_sequences!(
            b"\x1Ba",
            Sequence::Key(KeyCode::Char('a'), KeyModifiers::ALT),
            b"\x1Bz",
            Sequence::Key(KeyCode::Char('z'), KeyModifiers::ALT),
            b"\x1B5",
            Sequence::Key(KeyCode::Char('5'), KeyModifiers::ALT),
        );
    }

    #[test]
    fn esc_o_f1_to_f4() {
        test_sequences!(
            b"\x1BOP",
            Sequence::Key(KeyCode::F(1), KeyModifiers::empty()),
            b"\x1BOQ",
            Sequence::Key(KeyCode::F(2), KeyModifiers::empty()),
            b"\x1BOR",
            Sequence::Key(KeyCode::F(3), KeyModifiers::empty()),
            b"\x1BOS",
            Sequence::Key(KeyCode::F(4), KeyModifiers::empty()),
        );
    }

    #[test]
    fn single_byte() {
        test_sequences!(
            b"\r",
            Sequence::Key(KeyCode::Enter, KeyModifiers::empty()),
            b"\n",
            Sequence::Key(KeyCode::Enter, KeyModifiers::empty()),
            b"\t",
            Sequence::Key(KeyCode::Tab, KeyModifiers::empty()),
            b"\x7F",
            Sequence::Key(KeyCode::BackTab, KeyModifiers::empty()),
            b"\x1B",
            Sequence::Key(KeyCode::Esc, KeyModifiers::empty()),
            b"\0",
            Sequence::Key(KeyCode::Null, KeyModifiers::empty()),
        );
    }

    #[test]
    fn csi_special_keys() {
        test_sequences!(
            b"\x1B[D",
            Sequence::Key(KeyCode::Left, KeyModifiers::empty()),
            b"\x1B[C",
            Sequence::Key(KeyCode::Right, KeyModifiers::empty()),
            b"\x1B[A",
            Sequence::Key(KeyCode::Up, KeyModifiers::empty()),
            b"\x1B[B",
            Sequence::Key(KeyCode::Down, KeyModifiers::empty()),
            b"\x1B[H",
            Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
            b"\x1B[F",
            Sequence::Key(KeyCode::End, KeyModifiers::empty()),
            b"\x1B[Z",
            Sequence::Key(KeyCode::BackTab, KeyModifiers::empty()),
        );
    }

    #[test]
    fn csi_xterm_mouse() {
        test_sequences!(
            b"\x1B[<0;20;10;M",
            Sequence::Mouse(Mouse::Down(
                MouseButton::Left,
                20,
                10,
                KeyModifiers::empty()
            )),
            b"\x1B[<0;20;10;m",
            Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10, KeyModifiers::empty())),
        );
    }
}
