use anes::{KeyModifiers, Mouse, MouseButton, Sequence};

use crate::test_sequences;

// TODO Cover all cases, especially drag

#[test]
fn buttons() {
    test_sequences!(
        b"\x1B[<0;20;10;M",
        Sequence::Mouse(Mouse::Down(
            MouseButton::Left,
            20,
            10,
            KeyModifiers::empty()
        )),
        b"\x1B[<1;20;10;M",
        Sequence::Mouse(Mouse::Down(
            MouseButton::Middle,
            20,
            10,
            KeyModifiers::empty()
        )),
        b"\x1B[<2;20;10;M",
        Sequence::Mouse(Mouse::Down(
            MouseButton::Right,
            20,
            10,
            KeyModifiers::empty()
        )),
        b"\x1B[<0;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10, KeyModifiers::empty())),
        b"\x1B[<1;20;10;m",
        Sequence::Mouse(Mouse::Up(
            MouseButton::Middle,
            20,
            10,
            KeyModifiers::empty()
        )),
        b"\x1B[<2;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Right, 20, 10, KeyModifiers::empty())),
        b"\x1B[<64;20;10;m",
        Sequence::Mouse(Mouse::ScrollUp(20, 10, KeyModifiers::empty())),
        b"\x1B[<65;20;10;m",
        Sequence::Mouse(Mouse::ScrollDown(20, 10, KeyModifiers::empty())),
    );
}

#[test]
fn key_modifiers() {
    test_sequences!(
        b"\x1B[<4;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10, KeyModifiers::SHIFT)),
        b"\x1B[<8;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10, KeyModifiers::ALT)),
        b"\x1B[<16;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10, KeyModifiers::CONTROL)),
        b"\x1B[<12;20;10;m",
        Sequence::Mouse(Mouse::Up(
            MouseButton::Left,
            20,
            10,
            KeyModifiers::SHIFT | KeyModifiers::ALT
        )),
        b"\x1B[<20;20;10;m",
        Sequence::Mouse(Mouse::Up(
            MouseButton::Left,
            20,
            10,
            KeyModifiers::SHIFT | KeyModifiers::CONTROL
        )),
        b"\x1B[<24;20;10;m",
        Sequence::Mouse(Mouse::Up(
            MouseButton::Left,
            20,
            10,
            KeyModifiers::ALT | KeyModifiers::CONTROL
        )),
        b"\x1B[<28;20;10;m",
        Sequence::Mouse(Mouse::Up(
            MouseButton::Left,
            20,
            10,
            KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
        )),
    );
}
