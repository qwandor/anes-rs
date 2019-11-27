# Contributing guidelines

## Fuzzer

```sh
$ cargo install afl
$ cargo afl build
$ cargo afl fuzz -i fuzzer/in -o fuzzer/out target/debug/fuzz-parser-advance 
```
