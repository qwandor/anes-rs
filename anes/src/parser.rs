use std::collections::VecDeque;

use engine::{Engine, Provide};
pub use types::{KeyCode, KeyModifiers, Mouse, MouseButton, Sequence};

mod engine;
mod parsers;
pub(crate) mod types;

/// An ANSI escape sequence parser.
///
/// `Parser` implements the `Iterator<Item = Sequence>` trait, thus you can use the
/// `next()` method to consume all valid sequences with known meaning.
///
/// # Examples
///
/// Parse cursor position:
///
/// ```
/// use anes::parser::{Parser, Sequence};
///
/// let mut parser = Parser::default();
/// parser.advance(b"\x1B[20;10R", false);
///
/// assert_eq!(Some(Sequence::CursorPosition(10, 20)), parser.next());
/// assert!(parser.next().is_none());
/// ```
///
/// Parse keyboard event:
///
/// ```
/// use anes::parser::{KeyCode, KeyModifiers, Parser, Sequence};
///
/// let mut parser = Parser::default();
/// parser.advance("𐌼a".as_bytes(), false);
///
/// assert_eq!(Some(Sequence::Key(KeyCode::Char('𐌼'), KeyModifiers::empty())), parser.next());
/// assert_eq!(Some(Sequence::Key(KeyCode::Char('a'), KeyModifiers::empty())), parser.next());
/// assert!(parser.next().is_none());
/// ```
#[derive(Default)]
pub struct Parser {
    engine: Engine,
    provider: SequenceProvider,
}

impl Parser {
    /// Advances parser state machine with additional input data.
    ///
    /// # Arguments
    ///
    /// * `buffer` - input data (stdin in raw mode, etc.)
    /// * `more` - more input data available right now
    ///
    /// It's crucial to provide correct `more` value in order to receive `KeyCode::Esc` events
    /// as soon as possible.
    ///
    /// # Examples
    ///
    /// Esc key:
    ///
    /// ```
    /// use anes::parser::{KeyCode, KeyModifiers, Parser, Sequence};
    ///
    /// let mut parser = Parser::default();
    /// // User pressed Esc key & nothing else which means that there's no additional input available
    /// // aka no possible escape sequence = `KeyCode::Esc` dispatched.
    /// parser.advance(&[0x1b], false);
    ///
    /// assert_eq!(Some(Sequence::Key(KeyCode::Esc, KeyModifiers::empty())), parser.next());
    /// assert!(parser.next().is_none());
    /// ```
    ///
    /// Possible escape sequence:
    ///
    /// ```
    /// use anes::parser::{KeyCode, KeyModifiers, Parser, Sequence};
    ///
    /// let mut parser = Parser::default();
    /// // User pressed F1 = b"\x1BOP"
    ///
    /// // Every escape sequence starts with Esc (0x1b). There's more input available
    /// // aka possible escape sequence = `KeyCode::Esc` isn't dispatched.
    /// parser.advance(&[0x1b], true);
    /// assert!(parser.next().is_none());
    ///
    /// // Advance parser with rest of the sequence
    /// parser.advance(&[b'O', b'P'], false);
    /// assert_eq!(Some(Sequence::Key(KeyCode::F(1), KeyModifiers::empty())), parser.next());
    /// assert!(parser.next().is_none());
    /// ```
    pub fn advance(&mut self, buffer: &[u8], more: bool) {
        let len = buffer.len();
        for (idx, byte) in buffer.iter().enumerate() {
            self.engine
                .advance(&mut self.provider, *byte, idx < len - 1 || more);
        }
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
        parser.advance(&[b'a'], false);
        assert!(parser.next().is_some());
    }

    #[test]
    fn dispatch_esc_sequence() {
        let mut parser = Parser::default();
        parser.advance(&[b'\x1B'], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'a'], false);
        assert!(parser.next().is_some());
    }

    #[test]
    fn does_not_dispatch_esc_sequence_with_upper_case_o() {
        let mut parser = Parser::default();
        parser.advance(&[b'\x1B'], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'O'], true);
        assert!(parser.next().is_none());
    }

    #[test]
    fn dispatch_esc_with_upper_case_o_followed_by_char_as_single_sequence() {
        let mut parser = Parser::default();
        parser.advance(&[b'\x1B'], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'O'], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'P'], false);
        assert!(parser.next().is_some());
        assert!(parser.next().is_none());
    }

    #[test]
    fn dispatch_csi_sequence() {
        let mut parser = Parser::default();
        parser.advance(&[b'\x1B'], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'['], true);
        assert!(parser.next().is_none());
        parser.advance(&[b'D'], false);
        assert!(parser.next().is_some());
    }
}
