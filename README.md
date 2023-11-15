# serde-pyobject

PyO3's PyAny as a serde data format

## Usage

### Serialize `T: Serialize` into `&'py PyAny`:

```rust
use serde::Serialize;
use pyo3::{Python, types::{PyAny, PyDict}};

#[derive(Serialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a = A { a: 1, b: "test".to_string() };
    let obj: &PyAny = serde_pyobject::to_pyobject(py, &a).unwrap();
    assert!(obj.is_instance_of::<PyDict>());
});
```

### Deserialize `&'py PyAny` into `T: Deserialize<'de>`:

```rust
use serde::Deserialize;
use pyo3::{Python, types::{PyAny, PyDict}};
use serde_pyobject::{from_pyobject, pydict};

#[derive(Debug, PartialEq, Deserialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a: &PyDict = pydict! { py,
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

### Serialize

| [serde data model] | PyO3 type | Rust input | Python output |
|------------------|-----------|------------|---------------|
| `i8`, `i16`, `i32`, `i64`, `isize`, <br> `u8`, `u16`, `u32`, `u64`, `usize` | `PyLong` | `123` | `123` |
| `f32`, `f64` | `PyFloat` | `1.0` | `1.0` |
| `bool` | `PyBool` | `true` | `true` |
| `char`, `string` | `PyString` | `'a'`, `"test"` | `"a"`, `"test"` |
| option | `PyAny` [^1] | `None`, `Some(1)` | `None`, `1` |
| unit | `PyTuple` | `()` | `()` |
| unit struct | `PyTuple` | `struct Unit` | `()` |
| unit variant | `PyDict` | `E::A` in `enum E { A, B }` | `{ "E": "A" }` |
| newtype struct | `PyDict` | `A(32)` of `struct A(u8)` | `{ "A": 32 }` |
| newtype variant | `PyDict` | `E::N(41)` of `enum E { N(u8) }` | `{ "E": ("N", 41) }` | 
| seq | `PyList` | `vec![1, 2, 3]` | `[1, 2, 3]` |
| tuple | `PyTuple` | `(1, "test")` | `(1, "test")` |
| tuple struct | `PyDict` | `T(1, "a")` of `struct T(u32, String)` | `{ "T": (1, "a") }` |
| tuple variant | `PyDict` | - | - |
| map | `PyDict` | - | - |
| struct | `PyDict` | `A { a: 1, b: "test" }` of `struct A { a: u32, b: String }` | `{ "A": { "a": 1, "b": "test"} }` |
| struct variant | `PyDict` | - | - |

[^1]: `Some(value)` is serialized as `value`

## License

© 2023 Jij Inc.

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
