# rstm

[![crates.io](https://img.shields.io/crates/v/rstm.svg)](https://crates.io/crates/rstm)
[![docs.rs](https://docs.rs/rstm/badge.svg)](https://docs.rs/rstm)

[![clippy](https://github.com/FL03/rstm/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rstm/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rstm/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rstm/actions/workflows/rust.yml)

***

### _The library is currently in the early stages of development and is not yet ready for production use._

This library focuses on building concrete implementations for Turing Machines. 

## Features



## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rstm.git
cd rstm
```

```bash
cargo build --features full -r --workspace
```

## Usage

### Example

```rust
    extern crate rstm;

    fn main() -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
