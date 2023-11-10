use pyo3::{
    prelude::*,
    types::{PyDict, PyFloat, PyList, PyLong, PyString, PyTuple},
};
use serde::Serialize;

use serde_pyobject::*;

#[test]
fn serialize_string() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &'a').unwrap();
        assert!(obj.is_instance_of::<PyString>());

        let obj = as_pyobject(py, "test").unwrap();
        assert!(obj.is_instance_of::<PyString>());
    });
}

#[test]
fn serialize_long() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &1_u16).unwrap();
        assert!(obj.is_instance_of::<PyLong>());

        let obj = as_pyobject(py, &1_i64).unwrap();
        assert!(obj.is_instance_of::<PyLong>());

        let obj = as_pyobject(py, &1_i64).unwrap();
        assert!(obj.is_instance_of::<PyLong>());
    });
}

#[test]
fn serialize_float() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &3.1_f32).unwrap();
        assert!(obj.is_instance_of::<PyFloat>());

        let obj = as_pyobject(py, &-3.1_f64).unwrap();
        assert!(obj.is_instance_of::<PyFloat>());
    });
}

// Rust `None` is serialized to Python `None`, and `Some(value)` is serialized as `value` is serialized
#[test]
fn serialize_option() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &Option::<i32>::None).unwrap();
        assert!(obj.is(&py.None()));

        let obj = as_pyobject(py, &Some(1_i64)).unwrap();
        assert!(obj.is_instance_of::<PyLong>());
    });
}

// Rust `()` is serialized to Python `()`
#[test]
fn serialize_unit() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &()).unwrap();
        assert!(obj.is(PyTuple::empty(py)));
    });
}

#[derive(Serialize)]
struct UnitStruct;

// `Unit` is serialized as an empty tuple `()`
#[test]
fn serialize_unit_struct() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &UnitStruct {}).unwrap();
        assert!(obj.is_instance_of::<PyDict>());
        let value = obj
            .downcast::<PyDict>()
            .unwrap()
            .get_item("UnitStruct")
            .unwrap()
            .unwrap()
            .extract::<&PyTuple>()
            .unwrap();
        assert!(value.is(PyTuple::empty(py)));
    });
}

#[derive(Serialize)]
enum UnitVariant {
    A,
    B,
}

// Unit variant `E::A` is serialized as a dict `{ "E": "A" }`
#[test]
fn serialize_unit_variant() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &UnitVariant::A).unwrap();
        assert!(obj.is_instance_of::<PyDict>());
        let tag = obj
            .downcast::<PyDict>()
            .unwrap()
            .get_item("UnitVariant")
            .unwrap()
            .unwrap()
            .extract::<&str>()
            .unwrap();
        assert_eq!(tag, "A");

        let obj = as_pyobject(py, &UnitVariant::B).unwrap();
        assert!(obj.is_instance_of::<PyDict>());
        let tag = obj
            .downcast::<PyDict>()
            .unwrap()
            .get_item("UnitVariant")
            .unwrap()
            .unwrap()
            .extract::<&str>()
            .unwrap();
        assert_eq!(tag, "B");
    });
}

// TODO newtype struct

#[derive(Serialize)]
enum NewtypeVariant {
    N(u8),
}

#[test]
fn serialize_newtype_variant() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &NewtypeVariant::N(3)).unwrap();
        assert!(obj.is_instance_of::<PyDict>());
        let (tag, value) = obj
            .downcast::<PyDict>()
            .unwrap()
            .get_item("NewtypeVariant")
            .unwrap()
            .unwrap()
            .extract::<(&str, u8)>()
            .unwrap();
        assert_eq!(tag, "N");
        assert_eq!(value, 3);
    });
}

#[test]
fn serialize_seq() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &vec![1, 2, 3]).unwrap();
        assert!(obj.is_instance_of::<PyList>());
        let value = obj.downcast::<PyList>().unwrap();
        assert_eq!(value.get_item(0).unwrap().extract::<i32>().unwrap(), 1);
        assert_eq!(value.get_item(1).unwrap().extract::<i32>().unwrap(), 2);
        assert_eq!(value.get_item(2).unwrap().extract::<i32>().unwrap(), 3);
    });
}

#[test]
fn serialize_tuple() {
    Python::with_gil(|py| {
        let obj = as_pyobject(py, &(3, "test")).unwrap();
        assert!(obj.is_instance_of::<PyTuple>());
        let value = obj.downcast::<PyTuple>().unwrap();
        assert_eq!(value.get_item(0).unwrap().extract::<i32>().unwrap(), 3);
        assert_eq!(
            value.get_item(1).unwrap().extract::<&str>().unwrap(),
            "test"
        );
    });
}

// TODO tuple struct

// TODO tuple variant

// TODO map

#[derive(Serialize)]
struct Struct {
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
        let obj = as_pyobject(
            py,
            &Struct {
                a: 32,
                b: "test".to_string(),
            },
        )
        .unwrap();
        assert!(obj.is_instance_of::<PyDict>());
    });
}

// TODO struct variant
