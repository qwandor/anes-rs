use anes::{KeyModifiers, Mouse, MouseButton, Sequence};

use crate::test_sequences;

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

//#[test]
//#[cfg(all(feature = "parser", test))]
//fn special_key_codes() {
//    test_sequences!(
//        b"\x1B[1~",
//        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
//        b"\x1B[2~",
//        Sequence::Key(KeyCode::Insert, KeyModifiers::empty()),
//        b"\x1B[3~",
//        Sequence::Key(KeyCode::Delete, KeyModifiers::empty()),
//        b"\x1B[4~",
//        Sequence::Key(KeyCode::End, KeyModifiers::empty()),
//        b"\x1B[5~",
//        Sequence::Key(KeyCode::PageUp, KeyModifiers::empty()),
//        b"\x1B[6~",
//        Sequence::Key(KeyCode::PageDown, KeyModifiers::empty()),
//        b"\x1B[7~",
//        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
//        b"\x1B[8~",
//        Sequence::Key(KeyCode::End, KeyModifiers::empty()),
//        b"\x1B[11~",
//        Sequence::Key(KeyCode::F(1), KeyModifiers::empty()),
//        b"\x1B[12~",
//        Sequence::Key(KeyCode::F(2), KeyModifiers::empty()),
//        b"\x1B[13~",
//        Sequence::Key(KeyCode::F(3), KeyModifiers::empty()),
//        b"\x1B[14~",
//        Sequence::Key(KeyCode::F(4), KeyModifiers::empty()),
//        b"\x1B[15~",
//        Sequence::Key(KeyCode::F(5), KeyModifiers::empty()),
//        b"\x1B[17~",
//        Sequence::Key(KeyCode::F(6), KeyModifiers::empty()),
//        b"\x1B[18~",
//        Sequence::Key(KeyCode::F(7), KeyModifiers::empty()),
//        b"\x1B[19~",
//        Sequence::Key(KeyCode::F(8), KeyModifiers::empty()),
//        b"\x1B[20~",
//        Sequence::Key(KeyCode::F(9), KeyModifiers::empty()),
//        b"\x1B[21~",
//        Sequence::Key(KeyCode::F(10), KeyModifiers::empty()),
//        b"\x1B[23~",
//        Sequence::Key(KeyCode::F(11), KeyModifiers::empty()),
//        b"\x1B[24~",
//        Sequence::Key(KeyCode::F(12), KeyModifiers::empty()),
//    );
//}
//
