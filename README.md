# Garaga Noir Backend

Garaga Noir Backend is a Rust-based backend service for handling Garaga calldata generation Noir circuits. Calldata generation takes most of the frontend proving time. This library offloads the computation of calldata to the server.

## Features
- High-performance Rust backend
- Simple implementation
- Unit and integration tests

## Project Structure
- `src/` - Main source code
  - `main.rs` - Application entry point
  - `lib.rs` - Library module
  - `garaga.rs` - Core logic
- `tests/` - Test modules
- `Cargo.toml` - Rust project configuration

## Getting Started
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build
```sh
cargo build --release
```

### Run
```sh
cargo run
```

### Test
```sh
cargo test
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
MIT

## Authors
- mistcash

## Contact
For questions or support, open an issue in this repository.
