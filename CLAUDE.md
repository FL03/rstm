# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rstm** is a Rust framework for building and executing Turing machines. It implements foundational Turing machine concepts through a modular architecture supporting native, WebAssembly, and WASI targets.

- **MSRV**: 1.85.0
- **Edition**: 2024
- **License**: Apache-2.0

## Build & Test Commands

```bash
# Build workspace
cargo build --workspace

# Build with all features
cargo build --workspace --features full

# Run tests
cargo test --workspace

# Run tests with all features
cargo test --workspace --all-features

# Run a single test
cargo test --workspace test_name -- --exact

# Run clippy
cargo clippy --workspace --all-features

# Format code
cargo fmt --all

# Run example
cargo run --example tmh --features default,json,tracing

# Run benchmarks
cargo criterion --workspace --features full
```

## Workspace Structure

```
rstm/           # Main facade crate, re-exports everything
core/           # Core primitives: actors, rules, programs, motion
state/          # State management: State<Q>, Halt<Q, H>
traits/         # Common trait definitions
tape/           # Tape implementations (minimal)
macros/         # Procedural macros (rule!, ruleset!, program!)
```

### Dependency Flow

`rstm` → `rstm-core` → `rstm-state` → `rstm-traits`

## Key Abstractions

### Rule System (core/src/rules/)

Rules define Turing machine transitions: `(state, symbol) -> Direction(next_state, write_symbol)`

- **Head<Q, A>**: Current condition (state + symbol being read)
- **Tail<Q, A>**: Reaction (direction + next_state + write_symbol)
- **Rule<Q, A>**: Complete rule combining head and tail

### State Management (state/src/)

- **State<Q>**: Wrapper around state values
- **Halt<Q, H>**: Enum for halting (`Halt(H)`) vs stepping (`Step(Q)`)
- Traits: `RawState` (sealed), `Stateful`, `StateExt`, `Halting`

### Actors/Execution (core/src/actors/)

- **MovingHead** (alias `TMH`): Standard Turing machine with moving read/write head
- Located in `core/src/actors/drivers/tmh.rs`

### Programs (core/src/programs/)

- **InstructionSet**: Collection of rules
- **Program<Q, A>**: Full program with rules and initial state

## Macro DSL

```rust
// Single rule
rstm::rule! { (0, 0) -> Right(1, 0) }

// Multiple rules
rstm::ruleset! {
    (0, 0) -> Right(1, 0),
    (0, 1) -> Stay(-1, 1),
}

// Complete program
rstm::program! {
    #[default_state(0)]
    rules: {
        (0, 0) -> Right(1, 0),
        (0, 1) -> Left(-1, 1),
    };
}
```

## Feature Flags

- **default**: `["actors", "macros", "std"]`
- **full**: All features enabled
- **std/alloc**: Standard library / heap allocation
- **wasm/wasi**: WebAssembly targets
- **serde/json**: Serialization support
- **rayon**: Parallel processing

## Architecture Patterns

### Sealed Traits

Major trait hierarchies use sealing via `private!` macro to prevent external implementations:
- `RawHead` → `HeadRepr`
- `RawTail` → `TailRepr`
- `RawState` → `Stateful` → `StateExt`

### Module Organization

Each crate follows:
- `impls/` - Trait implementations
- `traits/` - Public trait definitions
- Preludes for convenient imports

### no_std Support

The crate supports `no_std` environments. Either `std` or `alloc` feature must be enabled (compile error guards prevent missing both).
