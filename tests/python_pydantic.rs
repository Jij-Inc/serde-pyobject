//! Pydantic Model Tests (Python → Rust)
//!
//! This test suite verifies deserialization support for Pydantic models.
//! Pydantic is a popular Python library for data validation and settings management
//! using Python type annotations.
//!
//! **Requirements**: These tests require the `pydantic_support` feature to be enabled.
//! Additionally, Pydantic must be installed in the Python environment.
//!
//! Pydantic models inherit from `pydantic.BaseModel` and provide runtime type checking,
//! data validation, and automatic conversion. The implementation uses the `model_dump()`
//! method to extract data from Pydantic models before deserialization.
//!
//! Each test:
//! 1. Defines a Pydantic model class inheriting from `BaseModel`
//! 2. Creates an instance of that model with validation
//! 3. Deserializes the Python object to a Rust struct via `from_pyobject`
//! 4. Verifies correctness by comparing with a Rust-originated value
//!
//! This enables seamless integration with Python codebases that use Pydantic for
//! data modeling and validation.

use pyo3::{ffi::c_str, prelude::*};
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

#[test]
fn check_pydantic_object() {
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
from pydantic import BaseModel
class MyClass(BaseModel):
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
