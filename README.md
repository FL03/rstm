# rstm

[![license](https://img.shields.io/crates/l/rstm.svg)](https://crates.io/crates/rstm)
[![crates.io](https://img.shields.io/crates/v/rstm.svg)](https://crates.io/crates/rstm)
[![docs.rs](https://docs.rs/rstm/badge.svg)](https://docs.rs/rstm)

[![clippy](https://github.com/FL03/rstm/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rstm/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rstm/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rstm/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is still settling in on a feel for the api.**_

Welcome to `rstm`! This crate provides a simple and easy-to-use interface for creating and executing Turing machines. The crate is designed to be flexible and extensible, allowing developers to create and execute a wide range of Turing machines. Furthermore, the crate focuses on efficiency and leverages feature-gating to reduce overhead.

## Getting Started

### From the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rstm.git
cd rstm
```

```bash
cargo build --all-features --workspace
```

#### _Run an example_

```bash
cargo run -f F --example {actor}
```

## Usage

### Rulesets

To faciliate the creation of rules for the machine, the crate provides a `ruleset!` macro. The macro mimics the
structure of the transition function $\delta$ defined by "On topological dynamics of Turing machines" by Petr Kůrka.

$$\delta : Q\times{A}\rarr{Q\times{A}\times{(0, \pm{1})}}$$

The syntax of the macro is as follows:

```rust
    ruleset![
        (state, symbol) -> Direction(next_state, next_symbol),
        ...
    ]
```

The macro expands into a `Vec<Rule>` where `Rule` is structure consisting of two other structures, namely: `Head<Q, S>` and the `Tail<Q, S>`. Each of these structures is a direct representation of the two sides of the transition function defined above

#### _Rules_

```rust
    pub struct Rule<Q, S> {
        pub head: Head<Q, S>,
        pub tail: Tail<Q, S>,
    }
```

where `Head` and `Tail` are defined as follows:

```rust
    pub struct Head<Q, S> {
        pub state: Q,
        pub symbol: S,
    }

    pub struct Tail<Q, S> {
        pub direction: Direction,
        pub state: Q,
        pub symbol: S,
    }
```

**Note:** the macro is hygenic, meaning developers will not need to import the `Direction` enum nor its variants in order to use the macro.

#### _Example usage_

The following example demonstrates the use of the `ruleset!` macro to define a set of rules for a three-state, two-symbol Turing machine.

```rust
    use rstm::ruleset;

    // define the ruleset for the machine
    let rules = ruleset![
        (0, 0) -> Right(1, 0),
        (0, 1) -> Stay(-1, 1),
        (1, 0) -> Left(0, 1),
        (1, 1) -> Right(-1, 0),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Right(1, 1),
    ];
```

### Examples

#### _Executing a program using an `Actor`_

```rust
    extern crate rstm;

    use rstm::{ruleset, Actor, Program, State};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt().with_target(false).init();
        // initialize the tape data
        let alpha = vec![0u8; 10];
        // initialize the state of the machine
        let initial_state = State::<isize>::default();
        // define the ruleset for the machine
        let rules = ruleset![
            (0, 0) -> Right(1, 0),
            (0, 1) -> Right(-1, 1),
            (1, 0) -> Right(0, 1),
            (1, 1) -> Right(-1, 0),
            (-1, 0) -> Left(0, 0),
            (-1, 1) -> Left(1, 1),
        ];
        // create a new program from the ruleset
        let program = Program::from_iter(rules);
        // create a new instance of the machine
        let tm = dbg!(Actor::new(alpha, initial_state, 0));
        // execute the program
        tm.execute(program).run()?;
        Ok(())
    }
```

## Contributing

Pull requests are welcome. Any improvements or modifactions should first be disccussed using a pull-request and/or by opening an issue. Additionally, please make sure to update tests as appropriate and to adhear to the feature gates.
