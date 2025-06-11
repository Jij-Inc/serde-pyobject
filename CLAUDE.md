# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build and Test
```bash
cargo build                    # Build the project
cargo test                     # Run all tests
cargo test test_name           # Run a specific test
cargo test --lib               # Run only library tests
cargo doc --open               # Generate and open documentation
```

### Code Quality
```bash
cargo fmt                      # Format code
cargo clippy                   # Run linter
cargo check                    # Fast compile check
```

## Project Architecture

This is a Rust crate that bridges PyO3 and serde, providing serialization/deserialization between Rust data structures and Python objects.

### Core Components

- **lib.rs**: Main module exports (`to_pyobject`, `from_pyobject`, `Error`)
- **ser.rs**: Serialization from Rust types to Python objects via serde
- **de.rs**: Deserialization from Python objects to Rust types via serde  
- **error.rs**: Custom error types for conversion failures
- **pylit.rs**: Convenience macros (`pydict!`, `pylist!`) for creating Python objects

### Key Design Patterns

The crate implements the serde data model mapping to PyO3 types:
- Rust structs → Python dictionaries
- Rust vectors → Python lists  
- Rust tuples → Python tuples
- Rust enums → Python dictionaries with variant names as keys
- Option types: `Some(T)` serializes as `T`, `None` as Python `None`

### Dependencies

- **pyo3**: ^0.23.0 (Python FFI bindings)
- **serde**: 1.0 (serialization framework)
- **Dev dependencies**: maplit, serde_json (for testing)

### Testing Strategy

Tests are organized in `tests/` directory:
- `to_json_to_pyobject.rs`: Cross-validation tests comparing direct serialization vs JSON roundtrip
- `check_revertible.rs`: Bidirectional conversion tests (serialize → deserialize → compare)

The crate uses Python's GIL (`Python::with_gil`) for all Python object operations.