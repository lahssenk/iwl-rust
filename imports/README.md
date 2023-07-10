# Imports

This chapter presents how to import libraries/modules/crates
in your project.

## Use modules from same crate
- colocated source code (internal/local dependency)
- no need to declare in Cargo.toml
- `mod` keyword to add Symbols into scope

## Use modules from standard library (core, std, alloc)
- no need to declare in Cargo.toml
- `use` keyword to add Symbols into scope

## Import modules from external crates
- need to declare in Cargo.toml -> `cargo add <name> [-p <workspacePackageName>]`
- `use` keyword to add Symbols into scope