# serde-pyobject

PyO3's PyAny as a serde data format

## Mapping between Python and serde data model

[serde data model](https://serde.rs/data-model.html) is a data model used in serde.

| serde data model | PyO3 type |
|------------------|-----------|
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `PyLong` |
| `f32`, `f64` | `PyFloat` |
| `bool` | `PyBool` |
| `char`, `string` | `PyString` |

## License

© 2023 Jij Inc.

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
