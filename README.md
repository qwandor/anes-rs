[![Stable Status][actions-stable-badge]][actions-link]
[![Beta Status][actions-beta-badge]][actions-link]
[![Nightly Status][actions-nightly-badge]][actions-link]
[![crates.io][crates-badge]][crates-link]
[![docs.rs][docs-badge]][docs-link]
[![MIT][mit-license-badge]][mit-license-link]
[![Apache 2.0][apache-license-badge]][apache-license-link]
![LOC][loc-badge]

# ANSI Escape Sequence

A Rust library which provides an ANSI escape sequences (or codes, whatever you like more).

## Terminal Support

Not all ANSI escape sequences are supported by all terminals. You can use the
[interactive-test](https://github.com/zrzka/anes-rs/tree/master/interactive-test) to test them.
Checkout the repository and then:
 
```bash
$ cd anes-rs
$ cargo run --bin interactive-test
``` 

## Examples

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
anes = "0.1"
```

</details>
<p></p>


An example how to retrieve the ANSI escape sequence as a `String`:

```rust
use anes::SaveCursorPosition;

fn main() {
    let string = format!("{}", SaveCursorPosition);
    assert_eq!(&string, "\x1B7");
}
```

An example how to use the ANSI escape sequence:

```rust
use std::io::{Result, Write};

use anes::execute;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    execute!(
        &mut stdout,
        anes::SaveCursorPosition,
        anes::MoveCursorTo(10, 10),
        anes::RestoreCursorPosition
    )?;
    Ok(())
}
```

## Motivation

There're couple of terminal crates like:

* [crossterm](https://github.com/crossterm-rs/crossterm),
* [termion](https://github.com/redox-os/termion),
* etc.

All these crates do share two pieces of code:

* ANSI escape sequences and
* input event parsers.

I think that it's a waste of resources and asked Timon (the `crossterm` crate maintainer) what he thinks
about a new crate as a building block for the `crossterm` and other crates. And here we
are ...

## License

The ANES crate is dual-licensed under [Apache 2.0][apache-license-link] and
[MIT][mit-license-link] terms.

Copyrights in the ANES project are retained by their contributors. No
copyright assignment is required to contribute to the ANES project.

[actions-stable-badge]: https://github.com/zrzka/anes-rs/workflows/stable/badge.svg
[actions-beta-badge]: https://github.com/zrzka/anes-rs/workflows/beta/badge.svg
[actions-nightly-badge]: https://github.com/zrzka/anes-rs/workflows/nightly/badge.svg
[actions-link]: https://github.com/zrzka/anes-rs/actions

[crates-badge]: https://img.shields.io/crates/v/anes.svg
[crates-link]: https://crates.io/crates/anes

[docs-badge]: https://docs.rs/anes/badge.svg
[docs-link]: https://docs.rs/anes

[mit-license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-license-link]: ./LICENSE-MIT
[apache-license-badge]: https://img.shields.io/badge/license-Apache2-blue.svg
[apache-license-link]: /LICENSE-APACHE

[loc-badge]: https://tokei.rs/b1/github/zrzka/anes-rs?category=code
