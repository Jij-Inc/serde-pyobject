pub mod error;
pub mod serializer;

#[cfg(test)]
mod tests {
    use crate::serializer::*;
    use pyo3::{prelude::*, types::PyString};

    #[test]
    fn test_as_py_object() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, "test").unwrap();
            assert!(obj.is_instance_of::<PyString>());
        });
    }
}
