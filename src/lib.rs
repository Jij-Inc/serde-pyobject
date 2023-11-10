pub mod error;
pub mod serializer;

#[cfg(test)]
mod tests {
    use crate::serializer::*;
    use pyo3::{
        prelude::*,
        types::{PyDict, PyString},
    };
    use serde::Serialize;

    #[test]
    fn serialize_string() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, "test").unwrap();
            assert!(obj.is_instance_of::<PyString>());
        });
    }

    #[derive(Serialize)]
    struct A {
        a: i32,
        b: String,
    }

    #[test]
    fn serialize_struct() {
        Python::with_gil(|py| {
            let obj = as_py_object(
                py,
                &A {
                    a: 32,
                    b: "test".to_string(),
                },
            )
            .unwrap();
            assert!(obj.is_instance_of::<PyDict>());
        });
    }
}
