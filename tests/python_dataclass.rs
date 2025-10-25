//! Python Dataclass Tests (Python → Rust)
//!
//! This test suite verifies deserialization support for Python dataclasses,
//! which are part of the Python standard library (Python 3.7+).
//!
//! Dataclasses provide a decorator-based way to automatically generate boilerplate
//! code for classes that primarily store data. They use the `@dataclass` decorator
//! and automatically create `__init__`, `__repr__`, and other methods.
//!
//! The implementation uses `dataclasses.asdict()` to convert dataclass instances
//! into dictionaries before deserialization, ensuring proper handling of all fields
//! including nested dataclasses.
//!
//! Each test:
//! 1. Defines a Python dataclass with `@dataclass` decorator
//! 2. Creates an instance of that dataclass
//! 3. Deserializes the Python object to a Rust struct via `from_pyobject`
//! 4. Verifies correctness by comparing with a Rust-originated value
//!
//! Coverage includes simple dataclasses and nested dataclass structures.

use pyo3::{ffi::c_str, prelude::*};
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

#[test]
fn check_dataclass_object() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        name: String,
        age: i32,
    }

    Python::attach(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
from dataclasses import dataclass
@dataclass
class MyClass:
    name: str
    age: int
"#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass(name="John", age=30)
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            name: "John".to_string(),
            age: 30,
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        println!("any: {:?}", any);

        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}

#[test]
fn check_dataclass_object_nested() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClassNested {
        name: String,
        age: i32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        my_class: MyClassNested,
    }

    Python::attach(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
from dataclasses import dataclass
@dataclass
class MyClassNested:
    name: str
    age: int

@dataclass
class MyClass:
    my_class: MyClassNested
"#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass(my_class=MyClassNested(name="John", age=30))
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            my_class: MyClassNested {
                name: "John".to_string(),
                age: 30,
            },
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        println!("any: {:?}", any);

        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}
