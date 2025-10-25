//! Python-Specific Type Tests (Python → Rust)
//!
//! This test suite verifies deserialization support for Python-specific types:
//!
//! - **Custom Python classes**: User-defined classes with `__dict__` attribute
//! - **Dataclasses**: Python standard library dataclasses (Python 3.7+)
//! - **Pydantic models**: Pydantic BaseModel subclasses (requires `pydantic_support` feature)
//!
//! Each test performs the following:
//!
//! 1. **Create**: Define and instantiate a Python object (dataclass, Pydantic model, etc.)
//! 2. **Deserialize**: Python object → Rust value (via `from_pyobject`)
//! 3. **Compare**: Verify the Rust value matches the expected structure
//!
//! **Test Strategy**: To verify correctness, each test also creates an equivalent Rust value,
//! serializes it to Python, and deserializes it back. Both deserialized results should match,
//! confirming that Python-specific types are correctly understood.
//!
//! These tests complement `check_revertible.rs` by focusing on Python-native data structures
//! rather than starting from Rust types.

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

#[cfg(feature = "pydantic_support")]
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
