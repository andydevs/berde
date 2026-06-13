# berde

A migration system for Serde data files. `berde` helps you evolve versioned JSON/YAML/TOML
(and other Serde-compatible) configuration formats by defining each schema version as its own
type and providing the tooling to migrate data between those versions.

The project is in early scaffolding stages: the core library (`crates/berde`) is still
placeholder code, and `demos/json_schema` shows the versioned-schema layout that `berde` is
being designed around.

## Installation

This project is not yet published to crates.io. To work with it locally, clone the repository
and build the workspace with Cargo (Rust 2024 edition / a recent stable toolchain):

```sh
git clone https://github.com/andydevs/berde.git
cd berde
cargo build
```

## Usage

`cargo build` and `cargo test` operate on the whole workspace, including the demo crates:

```sh
cargo build          # build all workspace crates
cargo test           # run all tests in the workspace
cargo test -p berde  # run tests for just the berde crate
```

See [`demos/json_schema`](demos/json_schema) for an example of how a versioned schema is laid
out: each version of a type lives in its own module under `src/versions/` (e.g. `v1.rs`,
`v2.rs`), and a build script generates the module declarations that expose them as
`versions::v1`, `versions::v2`, etc.
