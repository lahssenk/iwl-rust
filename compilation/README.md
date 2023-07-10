# Compilation

This chapter covers the compilation of rust binaries or libraries.

## TLDR
- Compile with rustc or `cargo build`
- Running or testing code will compile it (`cargo run`, `cargo test`)
- In order to build an optimized (perf) version (longer compile time), run `cargo build --release`
- In order to benefit from shorter compilation time during dev, run `cargo build` which defaults to a dev build profile