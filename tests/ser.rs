use pyo3::{
    prelude::*,
    types::{PyFloat, PyList, PyLong, PyString, PyTuple},
};
use serde::Serialize;

use serde_pyobject::*;

#[test]
fn serialize_string() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &'a').unwrap();
        assert!(obj.is_instance_of::<PyString>());

        let obj = to_pyobject(py, "test").unwrap();
        assert!(obj.is_instance_of::<PyString>());
    });
}

#[test]
fn serialize_long() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &1_u16).unwrap();
        assert!(obj.is_instance_of::<PyLong>());

        let obj = to_pyobject(py, &1_i64).unwrap();
        assert!(obj.is_instance_of::<PyLong>());

        let obj = to_pyobject(py, &1_i64).unwrap();
        assert!(obj.is_instance_of::<PyLong>());
    });
}

#[test]
fn serialize_float() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &3.1_f32).unwrap();
        assert!(obj.is_instance_of::<PyFloat>());

        let obj = to_pyobject(py, &-3.1_f64).unwrap();
        assert!(obj.is_instance_of::<PyFloat>());
    });
}

// Rust `None` is serialized to Python `None`, and `Some(value)` is serialized as `value` is serialized
#[test]
fn serialize_option() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &Option::<i32>::None).unwrap();
        assert!(obj.is(&py.None()));

        let obj = to_pyobject(py, &Some(1_i64)).unwrap();
        assert!(obj.is_instance_of::<PyLong>());
    });
}

// Rust `()` is serialized to Python `()`
#[test]
fn serialize_unit() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &()).unwrap();
        assert!(obj.is(PyTuple::empty(py)));
    });
}

#[derive(Serialize)]
struct UnitStruct;

// `Unit` is serialized as an empty tuple `()`
#[test]
fn serialize_unit_struct() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &UnitStruct {}).unwrap();
        let dict = pydict! {
            "UnitStruct" => PyTuple::empty(py)
        }
        .unwrap();
        assert!(obj.eq(dict).unwrap());
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
        let obj = to_pyobject(py, &UnitVariant::A).unwrap();
        assert!(obj.eq(pydict! { "UnitVariant" => "A" }.unwrap()).unwrap());

        let obj = to_pyobject(py, &UnitVariant::B).unwrap();
        assert!(obj.eq(pydict! { "UnitVariant" => "B" }.unwrap()).unwrap());
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
        let obj = to_pyobject(py, &NewtypeVariant::N(3)).unwrap();
        assert!(obj
            .eq(pydict! { "NewtypeVariant" => ("N", 3) }.unwrap())
            .unwrap());
    });
}

#[test]
fn serialize_seq() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &vec![1, 2, 3]).unwrap();
        assert!(obj.eq(PyList::new(py, [1, 2, 3])).unwrap());
    });
}

#[test]
fn serialize_tuple() {
    Python::with_gil(|py| {
        let obj = to_pyobject(py, &(3, "test")).unwrap();
        assert!(obj
            .eq(PyTuple::new(py, [3.into_py(py), "test".into_py(py)]))
            .unwrap());
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
        let obj = to_pyobject(
            py,
            &Struct {
                a: 32,
                b: "test".to_string(),
            },
        )
        .unwrap();
        assert!(obj
            .eq(pydict! {
                "Struct" => pydict!{
                    "a" => 32,
                    "b" => "test"
                }.unwrap()
            }
            .unwrap())
            .unwrap());
    });
}

// TODO struct variant
