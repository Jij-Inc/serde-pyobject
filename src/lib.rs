pub mod error;
pub mod serializer;

#[cfg(test)]
mod tests {
    use crate::serializer::*;
    use pyo3::{
        prelude::*,
        types::{PyDict, PyFloat, PyLong, PyString, PyTuple},
    };
    use serde::Serialize;

    #[test]
    fn serialize_string() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &'a').unwrap();
            assert!(obj.is_instance_of::<PyString>());

            let obj = as_py_object(py, "test").unwrap();
            assert!(obj.is_instance_of::<PyString>());
        });
    }

    #[test]
    fn serialize_long() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &1_u16).unwrap();
            assert!(obj.is_instance_of::<PyLong>());

            let obj = as_py_object(py, &1_i64).unwrap();
            assert!(obj.is_instance_of::<PyLong>());

            let obj = as_py_object(py, &1_i64).unwrap();
            assert!(obj.is_instance_of::<PyLong>());
        });
    }

    #[test]
    fn serialize_float() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &3.1_f32).unwrap();
            assert!(obj.is_instance_of::<PyFloat>());

            let obj = as_py_object(py, &-3.1_f64).unwrap();
            assert!(obj.is_instance_of::<PyFloat>());
        });
    }

    // Rust `None` is serialized to Python `None`, and `Some(value)` is serialized as `value` is serialized
    #[test]
    fn serialize_option() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &Option::<i32>::None).unwrap();
            assert!(obj.is(&py.None()));

            let obj = as_py_object(py, &Some(1_i64)).unwrap();
            assert!(obj.is_instance_of::<PyLong>());
        });
    }

    // Rust `()` is serialized to Python `()`
    #[test]
    fn serialize_unit() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &()).unwrap();
            assert!(obj.is(PyTuple::empty(py)));
        });
    }

    #[derive(Serialize)]
    struct Unit;

    // `Unit` is serialized as an empty tuple `()`
    #[test]
    fn serialize_unit_struct() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &Unit {}).unwrap();
            assert!(obj.is_instance_of::<PyDict>());
            let value = obj
                .downcast::<PyDict>()
                .unwrap()
                .get_item("Unit")
                .unwrap()
                .unwrap()
                .extract::<&PyTuple>()
                .unwrap();
            assert!(value.is(PyTuple::empty(py)));
        });
    }

    #[derive(Serialize)]
    enum E {
        A,
        B,
    }

    // Unit struct `E::A` is serialized as a dict `{ "E": "A" }`
    #[test]
    fn serialize_unit_variant() {
        Python::with_gil(|py| {
            let obj = as_py_object(py, &E::A).unwrap();
            assert!(obj.is_instance_of::<PyDict>());
            let tag = obj
                .downcast::<PyDict>()
                .unwrap()
                .get_item("E")
                .unwrap()
                .unwrap()
                .extract::<&str>()
                .unwrap();
            assert_eq!(tag, "A");

            let obj = as_py_object(py, &E::B).unwrap();
            assert!(obj.is_instance_of::<PyDict>());
            let tag = obj
                .downcast::<PyDict>()
                .unwrap()
                .get_item("E")
                .unwrap()
                .unwrap()
                .extract::<&str>()
                .unwrap();
            assert_eq!(tag, "B");
        });
    }

    // TODO newtype struct

    // TODO newtype variant

    // TODO seq

    // TODO tuple

    // TODO tuple struct

    // TODO tuple variant

    // TODO map

    #[derive(Serialize)]
    struct A {
        a: i32,
        b: String,
    }

    // Struct `A { a: 32, b: "test".to_string() }` is serialized as a dict of dict
    //
    // ```
    // {
    //   "A": {
    //      "a": 32,
    //      "b": "test"
    //   }
    // }
    // ```
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

    // TODO struct variant
}
