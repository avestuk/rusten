# README

A place to sharpen skills in case they get rusty. 

We follow the [good book](https://doc.rust-lang.org/book/) 'ere 

# Notes

## Cargo

- `Cargo new` creates a standard directory structure and a git repo   
- `Cargo build` builds a binary - it'll land in the `target/debug` directory
- `Cargo build --release` builds a binary for release - it takes longer but adds optimizations
- `Cargo run` builds and runs the binary
- `Cargo check` verifies that the binary compiles

## Assorted

- `expect()` will panic with a custom message 
- `match` will let us handle the error instead