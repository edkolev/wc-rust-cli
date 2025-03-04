# wc-rust-cli

A cli in Rust which mimics "wc"'s output.

This is a toy project with the goal of familiarization with Rust's basics.

It follows an old tradition: [wc-haskell-cli](https://github.com/edkolev/wc)

## Sample output compare to "wc"

Words:

```
~/Projects/wc-cli $ cargo run -q -- Cargo.*
      26 Cargo.lock
      11 Cargo.toml
      37 total
~/Projects/wc-cli $ wc -w Cargo.*
      26 Cargo.lock
      11 Cargo.toml
      37 total
```

Lines:
```
~/Projects/wc-rust-cli $ cargo run -q -- -l Cargo.*
       7 Cargo.lock
       6 Cargo.toml
      13 total
~/Projects/wc-rust-cli $ wc -l Cargo.*
       7 Cargo.lock
       6 Cargo.toml
      13 total
```
