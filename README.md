[![Stable Status][actions-stable-badge]][actions-link]
[![Beta Status][actions-beta-badge]][actions-link]
[![Nightly Status][actions-nightly-badge]][actions-link]
[![crates.io][crates-badge]][crates-link]
[![docs.rs][docs-badge]][docs-link]
[![MIT][mit-license-badge]][mit-license-link]
[![Apache 2.0][apache-license-badge]][apache-license-link]
![LOC][loc-badge]

# ANSI Escape Sequences provider & parser

This README file is for the `anes-rs` repository. If you're looking
for the user documentation, please, go to [`anes/README.md`](anes/README.md).

## Repository organisation

```text 
anes-rs
 ├ README.md        - this README
 ├ anes             - `anes` crate
 │  ├ benches       - `anes` crate benchmarks
 │  ├ examples      - `anes` crate examples
 │  ├ src           - `anes` crate source code
 │  ├ tests         - `anes` crate tests
 │  └ README.md     - `anes` crate README for crates.io site
 ├ fuzzer            - fuzzing binary targets
 ├ interactive-test  - interactive test for the `anes` crate
 └ scripts           - various scripts to run bechmarks, fuzzer, ...
```

### Benchmarks

You can run benchmarks with the `scripts/bench.sh` script. `criterion` output is
available in the `anes/target/criterion` folder. If you'd like to modify the
parser module (`anes/src/parser`) in any way, please, do:

* run benchmarks,
* modify the parser code,
* run benchmarks again.

Do not commit any change which degrades the parser performance. TIA!

### Fuzzer

You can start fuzzing with the `scripts/fuzz.sh` script. Feel free to stop fuzzing
any time with the `Ctrl C`, AFL allows you to continue with the `scripts/fuzz-continue.sh`
script.

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
