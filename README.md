[![Actions Status][actions-status-badge]][actions-status-link]
[![crates.io][crates-badge]][crates-link]
[![docs.rs][docs-badge]][docs-link]
[![MIT][mit-license-badge]][mit-license-link]
![LOC][loc-badge]

[actions-status-badge]: https://github.com/zrzka/anes-rs/workflows/anes-rs%20test/badge.svg
[actions-status-link]: https://github.com/zrzka/anes-rs/actions

[crates-badge]: https://img.shields.io/crates/v/anes.svg
[crates-link]: https://crates.io/crates/anes

[docs-badge]: https://docs.rs/anes/badge.svg
[docs-link]: https://docs.rs/anes

[mit-license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-license-link]: ./LICENSE

[loc-badge]: https://tokei.rs/b1/github/zrzka/anes-rs?category=code

# ANSI Escape Sequence

A Rust library which provides an ANSI escape sequences (or codes, whatever you like more).

Current status is **experimental**.

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

## Goals

* Provide ANSI escape sequences.
* Provide input events parser (2nd phase).

This crate does not and wont support execution or any other features not mentioned in
the goals section. It should be used as a building block for other crates like `crossterm` and
not as a replacement. Think about this when requesting new features.

## License

This project is licensed under the [MIT license](./LICENSE).
