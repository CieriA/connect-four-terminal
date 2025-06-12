# Connect 4 with Rust
A Connect 4 implementation on the terminal (pass-to-play), written in [Rust](https://www.rust-lang.org).

## Features
- Grid display
- Yellow / Red players
- I/O to play the game

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/tris-terminal
cd tris-terminal
cargo build --release
```

## Running the game
```bash
cargo run --release
```

## Controls
- 1-7 digit to choose a cell to place your cross/circle.

## Development notes
This is a test for a future Connect 4 project with a GUI,
to learn the logic behind the Connect 4' win condition.

This project doesn't use any external crate.

### Docs
To build the documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.

