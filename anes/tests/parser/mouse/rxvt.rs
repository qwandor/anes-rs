use anes::{KeyModifiers, Mouse, MouseButton, Sequence};

use crate::test_sequences;

// TODO Cover all cases

#[test]
fn buttons() {
    test_sequences!(
        b"\x1B[32;30;40;M",
        Sequence::Mouse(Mouse::Down(
            MouseButton::Left,
            30,
            40,
            KeyModifiers::empty()
        )),
    );
}
