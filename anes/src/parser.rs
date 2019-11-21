use std::collections::VecDeque;

use engine::{Engine, Perform};
use types::Event;

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
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.performer.next()
    }
}

#[derive(Default)]
struct Performer {
    events: VecDeque<Event>,
}

impl Iterator for Performer {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.pop_front()
    }
}

impl Perform for Performer {
    fn dispatch_char(&mut self, ch: char) {
        if let Some(event) = parsers::parse_char(ch) {
            self.events.push_back(event);
        }
    }

    fn dispatch_esc(&mut self, intermediates: &[u8], ignored_intermediates_count: usize, ch: char) {
        if let Some(event) = parsers::parse_esc(intermediates, ignored_intermediates_count, ch) {
            self.events.push_back(event);
        }
    }

    fn dispatch_csi(
        &mut self,
        parameters: &[i64],
        ignored_parameters_count: usize,
        intermediates: &[u8],
        ignored_intermediates_count: usize,
        ch: char,
    ) {
        if let Some(event) = parsers::parse_csi(
            parameters,
            ignored_parameters_count,
            intermediates,
            ignored_intermediates_count,
            ch,
        ) {
            self.events.push_back(event);
        }
    }
}
