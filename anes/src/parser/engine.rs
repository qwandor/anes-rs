//
// https://vt100.net/emu/dec_ansi_parser
//
// The parser is heavily inspired by the vte (https://crates.io/crates/vte) crate.
// Tried to use this crate, but it doesn't work for opposite way (terminal -> sequence),
// because there're couple of exceptions we have to handle and it doesn't make much
// sense to add them to the vte crate.
//
const MAX_PARAMETERS: usize = 30;
const DEFAULT_PARAMETER_VALUE: u64 = 0;
const MAX_UTF8_CODE_POINTS: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiIgnore,
    CsiParameter,
    CsiIntermediate,
    Utf8,
}

pub trait Perform {
    fn dispatch_char(&mut self, ch: char);

    fn dispatch_esc(&mut self, ch: char);

    fn dispatch_csi(&mut self, parameters: &[u64], ignored_count: usize, ch: char);
}

pub struct Engine {
    parameters: [u64; MAX_PARAMETERS],
    parameters_count: usize,
    parameter: u64,
    ignored_parameters_count: usize,
    state: State,
    utf8_points: [u8; MAX_UTF8_CODE_POINTS],
    utf8_points_count: usize,
    utf8_points_expected_count: usize,
}

impl Default for Engine {
    fn default() -> Self {
        Engine::new()
    }
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parameters: [DEFAULT_PARAMETER_VALUE; MAX_PARAMETERS],
            parameters_count: 0,
            parameter: DEFAULT_PARAMETER_VALUE,
            ignored_parameters_count: 0,
            state: State::Ground,
            utf8_points: [0; MAX_UTF8_CODE_POINTS],
            utf8_points_count: 0,
            utf8_points_expected_count: 0,
        }
    }

    fn set_state(&mut self, state: State) {
        if let State::CsiEntry = state {
            self.parameters_count = 0;
            self.parameter = DEFAULT_PARAMETER_VALUE;
            self.ignored_parameters_count = 0;
        }
        self.state = state;
    }

    fn handle_possible_esc(&mut self, performer: &mut dyn Perform, byte: u8, more: bool) -> bool {
        if byte != 0x1B {
            return false;
        }

        match (self.state, more) {
            // More input means possible Esc sequence, just switch state and wait
            (State::Ground, true) => self.set_state(State::Escape),

            // No more input means Esc key, dispatch it
            (State::Ground, false) => performer.dispatch_char('\x1B'),

            // More input means possible Esc sequence, dispatch the previous Esc char
            (State::Escape, true) => performer.dispatch_char('\x1B'),

            // No more input means Esc key, dispatch the previous & current Esc char
            (State::Escape, false) => {
                performer.dispatch_char('\x1B');
                performer.dispatch_char('\x1B');
                self.set_state(State::Ground);
            }

            // Discard any state
            // More input means possible Esc sequence
            (_, true) => self.set_state(State::Escape),

            // Discard any state
            // No more input means Esc key, dispatch it
            (_, false) => {
                performer.dispatch_char('\x1B');
                self.set_state(State::Ground);
            }
        }

        true
    }

    fn handle_possible_utf8_code_points(&mut self, performer: &mut dyn Perform, byte: u8) -> bool {
        if byte & 0b1000_0000 == 0b0000_0000 {
            performer.dispatch_char(byte as char);
            true
        } else if byte & 0b1110_0000 == 0b1100_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 2;
            self.set_state(State::Utf8);
            true
        } else if byte & 0b1111_0000 == 0b1110_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 3;
            self.set_state(State::Utf8);
            true
        } else if byte & 0b1111_1000 == 0b1111_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 4;
            self.set_state(State::Utf8);
            true
        } else {
            false
        }
    }

    fn advance_ground_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        if self.handle_possible_utf8_code_points(performer, byte) {
            return;
        }

        match byte {
            0x1B => unreachable!(),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // Print
            0x20..=0x7F => performer.dispatch_char(byte as char),

            _ => {}
        };
    }

    fn advance_escape_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::Escape);

        match byte {
            0x1B => unreachable!(),

            // Intermediate bytes to collect
            0x20..=0x2F => {
                self.set_state(State::EscapeIntermediate);
            }

            // Escape followed by '[' (0x5B)
            //   -> CSI sequence start
            0x5B => self.set_state(State::CsiEntry),

            // Escape sequence final character
            0x30..=0x4F | 0x51..=0x57 | 0x59 | 0x5A | 0x5C | 0x60..=0x7E => {
                performer.dispatch_esc(byte as char);
                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_escape_intermediate_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::Escape);

        match byte {
            0x1B => unreachable!(),

            // Intermediate bytes to collect
            0x20..=0x2F => {}

            // Escape followed by '[' (0x5B)
            //   -> CSI sequence start
            0x5B => self.set_state(State::CsiEntry),

            // Escape sequence final character
            0x30..=0x5A | 0x5C..=0x7E => {
                performer.dispatch_esc(byte as char);
                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_entry_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::CsiEntry);

        match byte {
            0x1B => unreachable!(),

            // Semicolon = parameter delimiter
            0x3B => {
                self.parameters[self.parameters_count] = self.parameter;
                self.parameters_count += 1;
                self.set_state(State::CsiParameter);
            }

            // '0' ..= '9' = parameter value
            0x30..=0x39 => {
                self.parameter = (byte as u64) - 0x30;
                self.set_state(State::CsiParameter);
            }

            0x3A => self.set_state(State::CsiIgnore),

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                performer.dispatch_csi(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Collect rest as parameters
            _ => {
                if self.parameters_count < MAX_PARAMETERS {
                    self.parameters[self.parameters_count] = byte as u64;
                    self.parameters_count += 1;
                } else {
                    self.ignored_parameters_count += 1;
                }
                self.parameter = DEFAULT_PARAMETER_VALUE;
            }
        };
    }

    fn advance_csi_ignore_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::CsiIgnore);

        match byte {
            0x1B => unreachable!(),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x20..=0x3F | 0x7F => {}

            0x40..=0x7E => self.set_state(State::Ground),

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_parameter_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::CsiParameter);

        match byte {
            0x1B => unreachable!(),

            // '0' ..= '9' = parameter value
            0x30..=0x39 => {
                self.parameter = self.parameter.saturating_mul(10);
                self.parameter = self.parameter.saturating_add((byte as u64) - 0x30);
            }

            // Semicolon = parameter delimiter
            0x3B => {
                if self.parameters_count < MAX_PARAMETERS {
                    self.parameters[self.parameters_count] = self.parameter;
                    self.parameters_count += 1;
                } else {
                    self.ignored_parameters_count += 1;
                }
                self.parameter = DEFAULT_PARAMETER_VALUE;
            }

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                if self.parameters_count < MAX_PARAMETERS {
                    self.parameters[self.parameters_count] = self.parameter;
                    self.parameters_count += 1;
                } else {
                    self.ignored_parameters_count += 1;
                }
                self.parameter = DEFAULT_PARAMETER_VALUE;

                performer.dispatch_csi(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Intermediates to collect
            0x20..=0x2F => {
                if self.parameters_count < MAX_PARAMETERS {
                    self.parameters[self.parameters_count] = self.parameter;
                    self.parameters_count += 1;
                } else {
                    self.ignored_parameters_count += 1;
                }
                self.parameter = DEFAULT_PARAMETER_VALUE;

                self.set_state(State::CsiIntermediate);
            }

            // Ignore
            0x3A | 0x3C..=0x3F => self.set_state(State::CsiIgnore),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_intermediate_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert_eq!(self.state, State::CsiIntermediate);

        match byte {
            0x1B => unreachable!(),

            // Intermediates to collect
            0x20..=0x2F => {}

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                performer.dispatch_csi(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => performer.dispatch_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        }
    }

    fn advance_utf8_state(&mut self, performer: &mut dyn Perform, byte: u8) {
        assert!(self.utf8_points_count < MAX_UTF8_CODE_POINTS);
        assert!(self.utf8_points_count > 0);

        if byte & 0b1100_0000 != 0b1000_0000 {
            self.set_state(State::Ground);
            return;
        }

        self.utf8_points[self.utf8_points_count] = byte;
        self.utf8_points_count += 1;

        if self.utf8_points_count == self.utf8_points_expected_count {
            if let Some(ch) = std::str::from_utf8(&self.utf8_points[..self.utf8_points_count])
                .ok()
                .and_then(|s| s.chars().next())
            {
                performer.dispatch_char(ch);
            }
            self.set_state(State::Ground);
        }
    }

    pub fn advance(&mut self, performer: &mut dyn Perform, byte: u8, more: bool) {
        // eprintln!("advance: {:?} {} {}", self.state, byte, more);

        if self.handle_possible_esc(performer, byte, more) {
            return;
        }

        match self.state {
            State::Ground => self.advance_ground_state(performer, byte),
            State::Escape => self.advance_escape_state(performer, byte),
            State::EscapeIntermediate => self.advance_escape_intermediate_state(performer, byte),
            State::CsiEntry => self.advance_csi_entry_state(performer, byte),
            State::CsiIgnore => self.advance_csi_ignore_state(performer, byte),
            State::CsiParameter => self.advance_csi_parameter_state(performer, byte),
            State::CsiIntermediate => self.advance_csi_intermediate_state(performer, byte),
            State::Utf8 => self.advance_utf8_state(performer, byte),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esc_char() {
        let mut engine = Engine::default();
        let mut performer = CharPerformer::default();

        // No more input means that the Esc character should be dispatched immediately
        engine.advance(&mut performer, 0x1B, false);
        assert_eq!(performer.chars, &['\x1B']);

        // There's more input so the machine should wait before dispatching Esc character
        engine.advance(&mut performer, 0x1B, true);
        assert_eq!(performer.chars, &['\x1B']);

        // Another Esc character, but no more input, machine should dispatch the postponed Esc
        // character and the new one too.
        engine.advance(&mut performer, 0x1B, false);
        assert_eq!(performer.chars, &['\x1B', '\x1B', '\x1B']);
    }

    #[test]
    fn esc_without_intermediates() {
        let mut engine = Engine::default();
        let mut performer = EscPerformer::default();

        let input = b"\x1B0\x1B~";
        advance(&mut engine, &mut performer, input, false);

        assert_eq!(performer.chars.len(), 2);

        assert_eq!(performer.chars[0], '0');

        assert_eq!(performer.chars[1], '~');
    }

    #[test]
    fn csi_without_parameters() {
        let mut engine = Engine::default();
        let mut performer = CsiPerformer::default();

        let input = b"\x1B\x5Bm";
        advance(&mut engine, &mut performer, input, false);

        assert_eq!(performer.parameters.len(), 1);
        assert_eq!(performer.parameters[0], &[]);
        assert_eq!(performer.chars.len(), 1);
        assert_eq!(performer.chars[0], 'm');
    }

    #[test]
    fn csi_with_two_default_parameters() {
        let mut engine = Engine::default();
        let mut performer = CsiPerformer::default();

        let input = b"\x1B\x5B;m";
        advance(&mut engine, &mut performer, input, false);

        assert_eq!(performer.parameters.len(), 1);
        assert_eq!(
            performer.parameters[0],
            &[DEFAULT_PARAMETER_VALUE, DEFAULT_PARAMETER_VALUE]
        );
        assert_eq!(performer.chars.len(), 1);
        assert_eq!(performer.chars[0], 'm');
    }

    #[test]
    fn csi_with_trailing_semicolon() {
        let mut engine = Engine::default();
        let mut performer = CsiPerformer::default();

        let input = b"\x1B\x5B123;m";
        advance(&mut engine, &mut performer, input, false);

        assert_eq!(performer.parameters.len(), 1);
        assert_eq!(performer.parameters[0], &[123, DEFAULT_PARAMETER_VALUE]);
        assert_eq!(performer.chars.len(), 1);
        assert_eq!(performer.chars[0], 'm');
    }

    #[test]
    fn csi_max_parameters() {
        let mut engine = Engine::default();
        let mut performer = CsiPerformer::default();

        let input = b"\x1B\x5B1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24;25;26;27;28;29;30m";
        advance(&mut engine, &mut performer, input, false);

        assert_eq!(performer.parameters.len(), 1);
        assert_eq!(performer.parameters[0].len(), MAX_PARAMETERS);
        assert_eq!(
            performer.parameters[0],
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30
            ]
        );
        assert_eq!(performer.chars.len(), 1);
        assert_eq!(performer.chars[0], 'm');
    }

    #[test]
    fn test_parse_utf8_character() {
        let mut engine = Engine::default();
        let mut performer = CharPerformer::default();

        advance(&mut engine, &mut performer, &['a' as u8], false);
        assert_eq!(performer.chars.len(), 1);
        assert_eq!(performer.chars[0], 'a');

        advance(&mut engine, &mut performer, &[0xC3, 0xB1], false);
        assert_eq!(performer.chars.len(), 2);
        assert_eq!(performer.chars[1], 'ñ');

        advance(&mut engine, &mut performer, &[0xE2, 0x81, 0xA1], false);
        assert_eq!(performer.chars.len(), 3);
        assert_eq!(performer.chars[2], '\u{2061}');

        advance(
            &mut engine,
            &mut performer,
            &[0xF0, 0x90, 0x8C, 0xBC],
            false,
        );
        assert_eq!(performer.chars.len(), 4);
        assert_eq!(performer.chars[3], '𐌼');
    }

    fn advance(engine: &mut Engine, performer: &mut dyn Perform, bytes: &[u8], more: bool) {
        let len = bytes.len();

        for (i, byte) in bytes.iter().enumerate() {
            engine.advance(performer, *byte, i < len - 1 || more);
        }
    }

    #[derive(Default)]
    struct CharPerformer {
        chars: Vec<char>,
    }

    impl Perform for CharPerformer {
        fn dispatch_char(&mut self, ch: char) {
            self.chars.push(ch);
        }

        fn dispatch_esc(&mut self, _ch: char) {}

        fn dispatch_csi(&mut self, _parameters: &[u64], _ignored_count: usize, _ch: char) {}
    }

    #[derive(Default)]
    struct CsiPerformer {
        parameters: Vec<Vec<u64>>,
        chars: Vec<char>,
    }

    impl Perform for CsiPerformer {
        fn dispatch_char(&mut self, _ch: char) {}

        fn dispatch_esc(&mut self, _ch: char) {}

        fn dispatch_csi(&mut self, parameters: &[u64], _ignored_count: usize, ch: char) {
            self.parameters.push(parameters.to_vec());
            self.chars.push(ch);
        }
    }

    #[derive(Default)]
    struct EscPerformer {
        chars: Vec<char>,
    }

    impl Perform for EscPerformer {
        fn dispatch_char(&mut self, _ch: char) {}

        fn dispatch_esc(&mut self, ch: char) {
            self.chars.push(ch);
        }

        fn dispatch_csi(&mut self, _parameters: &[u64], _ignored_count: usize, _ch: char) {}
    }
}
