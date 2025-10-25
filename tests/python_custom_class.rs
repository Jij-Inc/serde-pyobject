//! Custom Python Class Tests (Python → Rust)
//!
//! This test suite verifies deserialization support for custom Python classes
//! that use the `__dict__` attribute to store instance data.
//!
//! These are user-defined Python classes created with the `class` keyword and
//! `__init__` method, which is the most basic and common way to define classes
//! in Python.
//!
//! Each test:
//! 1. Defines a Python class with `__init__` method
//! 2. Creates an instance of that class
//! 3. Deserializes the Python object to a Rust struct via `from_pyobject`
//! 4. Verifies correctness by comparing with a Rust-originated value
//!
//! This ensures that basic Python objects can be seamlessly deserialized into
//! Rust structures, enabling interoperability with user-defined Python types.

use pyo3::{ffi::c_str, prelude::*};
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

#[test]
fn check_python_object() {
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
class MyClass:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
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
MyClass("John", 30)
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
        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}
