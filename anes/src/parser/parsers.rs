use super::types::{KeyCode, KeyModifiers, Mouse, MouseButton, Sequence};

pub(crate) fn parse_char(ch: char, esc_o: bool) -> Option<Sequence> {
    if esc_o {
        return match ch {
            'P'..='S' => Some(Sequence::Key(
                KeyCode::F(ch as u8 - b'P' + 1),
                KeyModifiers::empty(),
            )),
            _ => None,
        };
    }

    let code = match ch {
        '\r' | '\n' => KeyCode::Enter,
        '\t' => KeyCode::Tab,
        '\x7F' => KeyCode::BackTab,
        '\x1B' => KeyCode::Esc,
        '\0' => KeyCode::Null,
        _ => KeyCode::Char(ch),
    };
    Some(Sequence::Key(code, KeyModifiers::empty()))
}

pub(crate) fn parse_esc(
    _intermediates: &[u8],
    _ignored_intermediates_count: usize,
    ch: char,
) -> Option<Sequence> {
    // EscO[P-S] is handled in the Performer, see parse_char & esc_o argument
    // No need to handle other cases here? It's just Alt+$char
    Some(Sequence::Key(KeyCode::Char(ch), KeyModifiers::ALT))
}

pub(crate) fn parse_csi(
    parameters: &[u64],
    _ignored_parameters_count: usize,
    _intermediates: &[u8],
    _ignored_intermediates_count: usize,
    ch: char,
) -> Option<Sequence> {
    let seq = match ch {
        'D' => Sequence::Key(KeyCode::Left, KeyModifiers::empty()),
        'C' => Sequence::Key(KeyCode::Right, KeyModifiers::empty()),
        'A' => Sequence::Key(KeyCode::Up, KeyModifiers::empty()),
        'B' => Sequence::Key(KeyCode::Down, KeyModifiers::empty()),
        'H' => Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        'F' => Sequence::Key(KeyCode::End, KeyModifiers::empty()),
        'Z' => Sequence::Key(KeyCode::BackTab, KeyModifiers::empty()),
        _ if !parameters.is_empty() => {
            if parameters[0] == b'<' as i64 {
                return parse_csi_xterm_mouse(parameters, ch);
            }
            return None;
        }
        _ => return None,
    };

    Some(seq)
}

pub(crate) fn parse_csi_xterm_mouse(parameters: &[u64], ch: char) -> Option<Sequence> {
    // ESC [ < Cb ; Cx ; Cy (;) (M or m)

    if parameters.len() < 4 {
        return None;
    }

    let cb = parameters[1] as u8;
    let cx = parameters[2] as u16;
    let cy = parameters[3] as u16;

    let up = match ch {
        'm' => true,
        'M' => false,
        _ => return None,
    };

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let mouse = if cb & 0b0100_0000 == 0b0100_0000 {
        if cb & 0b0000_0001 == 0b0000_0001 {
            Mouse::ScrollDown(cx, cy, modifiers)
        } else {
            Mouse::ScrollUp(cx, cy, modifiers)
        }
    } else {
        let drag = cb & 0b0010_0000 == 0b0010_0000;

        match (cb & 0b111, up, drag) {
            (0, true, _) => Mouse::Up(MouseButton::Left, cx, cy, modifiers),
            (0, false, false) => Mouse::Down(MouseButton::Left, cx, cy, modifiers),
            (0, false, true) => Mouse::Drag(MouseButton::Left, cx, cy, modifiers),
            (1, true, _) => Mouse::Up(MouseButton::Middle, cx, cy, modifiers),
            (1, false, false) => Mouse::Down(MouseButton::Middle, cx, cy, modifiers),
            (1, false, true) => Mouse::Drag(MouseButton::Middle, cx, cy, modifiers),
            (2, true, _) => Mouse::Up(MouseButton::Right, cx, cy, modifiers),
            (2, false, false) => Mouse::Down(MouseButton::Right, cx, cy, modifiers),
            (2, false, true) => Mouse::Drag(MouseButton::Right, cx, cy, modifiers),
            _ => return None,
        }
    };

    Some(Sequence::Mouse(mouse))
}
