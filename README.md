# rstm

[![crates.io](https://img.shields.io/crates/v/rstm?style=for-the-badge&logo=rust)](https://crates.io/crates/rstm)
[![docs.rs](https://img.shields.io/docsrs/rstm?style=for-the-badge&logo=docs.rs)](https://docs.rs/rstm)
[![GitHub License](https://img.shields.io/github/license/FL03/rstm?style=for-the-badge&logo=github)](LICENSE)

***

Welcome to `rstm`! This crate provides a simple and easy-to-use interface for creating and executing Turing machines. The crate is designed to be flexible and extensible, allowing developers to create and execute a wide range of Turing machines. Furthermore, the crate focuses on efficiency and leverages feature-gating to reduce overhead.

## Rules

```rust
    pub struct Rule<Q, A> {
        pub head: Head<Q, A>,
        pub tail: Tail<Q, A>,
    }
```

where `Head` and `Tail` are defined as follows:

```rust
    pub struct Head<Q, A> {
        pub state: Q,
        pub symbol: A,
    }

    pub struct Tail<Q, A> {
        pub direction: Direction,
        pub next_state: Q,
        pub write_symbol: A,
    }
```

### Serialization

Enabling the `serde` feature will allow for serialization and deserialization of the `Rule` and other implementations within the crate. That being said, the serialization of the `Rule` macro is notable for the fact that it flattens both the `head` and `tail` fields, resulting in a more compact representation.

## `rule!`, `ruleset!`, and other rule-based macros

Researchers have simplified the definition of a Turing machine, boiling it down into a dynamical system defined by a set of states, symbols, and rules. The rules define the behavior of the machine, dictating how it transitions from one state to another based on the current symbol being read. More specifically, the transition function $\delta$ where:

$$
\delta: Q\times{A}\rightarrow{Q}\times{A}\times{\lbrace\pm{1},0\rbrace}
$$

as defined within the paper [On the Topological Dynamics of Turing Machines](https://doi.org/10.1016/S0304-3975(96)00025-4) by Petr Kůrka. Therefore, we allow any rule-based procedural macros within the scope of `rstm` to follow the following syntax:

```ignore
(state, symbol) -> Direction(next_state, next_symbol)
```

**Note:** the macros are hygenic, in the fact that they do not require the user to import any variants, traits, or other types into scope.

### Examples

For more examples visit the [examples](rstm/examples) directory.

#### **Example #1**: Using the `rule!` macro

The following example demonstrates the use of the `rule!` macro to define a single rule:

```rust
    rstm::rule! { (0, 0) -> Right(1, 0) }
```

#### **Example #2**: Using the `ruleset!` macro

The following example demonstrates the use of the `rules!` macro to define a set of rules:

```rust
    rstm::ruleset! {
        (0, 0) -> Right(1, 0),
        (0, 1) -> Stay(-1, 1),
        (1, 0) -> Left(0, 1),
        (1, 1) -> Right(-1, 0),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Right(1, 1),
    }
```

#### **Example #3**: Using the `program!` macro

The following example demonstrates the use of the `program!` macro to define a set of rules for a three-state, two-symbol Turing machine.

```rust
    // define the ruleset for the machine
    rstm::program! {
        #[default_state(0)] // optional
        rules: {
            (0, 0) -> Right(1, 0),
            (0, 1) -> Stay(-1, 1),
            (1, 0) -> Left(0, 1),
            (1, 1) -> Right(-1, 0),
            (-1, 0) -> Right(0, 0),
            (-1, 1) -> Right(<i8>::MAX, 1),
        };
    }
```

#### **Example #4**: Putting it all together with the `TMH` implementation

```rust
    extern crate rstm;

    use rstm::actors::TMH;
    use rstm::rules::Program;

    fn main() -> rstm::Result<()> {
        // initialize the logger
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_target(false)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .init();
        tracing::info!("Welcome to rstm!");
        // define some input for the machine
        let input = [0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
        // initialize the state of the machine
        let initial_state: isize = 0;
        // define the ruleset for the machine
        let program: Program<isize, usize> = rstm::program! {
            #[default_state(initial_state)]
            rules: {
                (0, 0) -> Right(1, 0),
                (0, 1) -> Left(-1, 1),
                (1, 0) -> Right(0, 1),
                (1, 1) -> Right(-1, 0),
                (-1, 0) -> Left(<isize>::MAX, 0),
                (-1, 1) -> Left(1, 1),
            };
        };
        // create a new instance of the machine
        let tm = TMH::new(initial_state, input.to_vec());
        // execute the program
        dbg!(tm).execute(program).run()?;
        Ok(())
    }
```

## Getting Started

For a more detailed guide on getting started, please refer to the [QUICKSTART.md](QUICKSTART.md) file.

### Building from the source

To build the project from source, start by cloning the repository:

```bash
git clone https://github.com/FL03/rstm.git
```

before switching into the project directory:

```bash
cd rstm
```

and building the project targeting the desired feature set:

```bash
cargo build --workspace --features default
```

#### _Run an example_

```bash
cargo run -f F --example {actor}
```

## Usage

To add `rstm` to your Rust project, run the following command:

```bash
cargo add rstm --features macros
```

or, manually include it in your `Cargo.toml` file as such:

```toml
[dependencies.rstm]
version = "0.1.x"
features = [
    "default",
]
```

## Contributing

Contributions are welcome! For more information visit the [CONTRIBUTING.md](CONTRIBUTING.md) file.
