# serde-pyobject


[![crate](https://img.shields.io/crates/v/serde-pyobject.svg)](https://crates.io/crates/serde-pyobject) 
[![docs.rs](https://docs.rs/serde-pyobject/badge.svg)](https://docs.rs/serde-pyobject)

PyO3's PyAny as a serde data format

## Usage

### Serialize `T: Serialize` into `&'py PyAny`:

```rust
use serde::Serialize;
use pyo3::{Python, Bound, types::{PyAny, PyAnyMethods, PyDict}};
use serde_pyobject::{to_pyobject, pydict};

#[derive(Serialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a = A { a: 1, b: "test".to_string() };
    let obj: Bound<PyAny> = to_pyobject(py, &a).unwrap();
    assert!(obj.eq(pydict! { py, "a" => 1, "b" => "test" }.unwrap()).unwrap());
});
```

### Deserialize `&'py PyAny` into `T: Deserialize<'de>`:

```rust
use serde::Deserialize;
use pyo3::{Python, Bound, types::{PyAny, PyAnyMethods, PyDict}};
use serde_pyobject::{from_pyobject, pydict};

#[derive(Debug, PartialEq, Deserialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a: Bound<PyDict> = pydict! { py,
      "a" => 1,
      "b" => "test"
    }
    .unwrap();
    let a: A = from_pyobject(a).unwrap();
    assert_eq!(a, A { a: 1, b: "test".to_string() });
});
```

## Mapping between Python and [serde data model]

[serde data model]: https://serde.rs/data-model.html

| [serde data model] | PyO3 type | Rust | Python |
|------------------|-----------|------------|---------------|
| `i8`, `i16`, `i32`, `i64`, `isize`, <br> `u8`, `u16`, `u32`, `u64`, `usize` | `PyLong` | `123` | `123` |
| `f32`, `f64` | `PyFloat` | `1.0` | `1.0` |
| `bool` | `PyBool` | `true` | `true` |
| `char`, `string` | `PyString` | `'a'`, `"test"` | `"a"`, `"test"` |
| option | `PyAny` [^1] | `None`, `Some(1)` | `None`, `1` |
| unit | `PyTuple` | `()` | `()` |
| unit struct | `PyTuple` | `struct Unit` | `()` |
| unit variant | `PyDict` | `E::A` in `enum E { A, B }` | `"A"` |
| newtype struct | `PyDict` | `A(32)` of `struct A(u8)` | `32` |
| newtype variant | `PyDict` | `E::N(41)` of `enum E { N(u8) }` | `{ "N": 41 }` | 
| seq | `PyList` | `vec![1, 2, 3]` | `[1, 2, 3]` |
| tuple | `PyTuple` | `(1, "test")` | `(1, "test")` |
| tuple struct | `PyDict` | `T(1, "a")` of `struct T(u32, String)` | `(1, "a")` |
| tuple variant | `PyDict` | `E::S(1, 2)` of `enum E { S(u8, u8) }` | `{ "S": (1, 2) }` |
| map | `PyDict` | `hashmap!{ "a".to_string() => 1, "b".to_string() => 2 }` | `{ "a": 1, "b": 2 }` |
| struct | `PyDict` | `A { a: 1, b: "test" }` of `struct A { a: u32, b: String }` | `{ "a": 1, "b": "test"}` |
| struct variant | `PyDict` | `E::S { r: 1, g: 2, b: 3 }` of `enum E { S { r: u8, g: u8, b: u8 } }` | `{ "S": { "r": 1, "g": 2, "b": 3 } }` |

[^1]: `Some(value)` is serialized as `value`

## License

© 2023 Jij Inc.

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
