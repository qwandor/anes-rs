use super::types::Event;

pub(crate) fn parse_char(_ch: char) -> Option<Event> {
    None
}

pub(crate) fn parse_esc(
    _intermediates: &[u8],
    _ignored_intermediates_count: usize,
    _ch: char,
) -> Option<Event> {
    None
}

pub(crate) fn parse_csi(
    _parameters: &[i64],
    _ignored_parameters_count: usize,
    _intermediates: &[u8],
    _ignored_intermediates_count: usize,
    _ch: char,
) -> Option<Event> {
    None
}
