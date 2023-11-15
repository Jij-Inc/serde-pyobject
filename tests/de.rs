use pyo3::{prelude::*, types::*};
use serde::Deserialize;
use serde_pyobject::{from_pyobject, pydict};
use std::collections::BTreeMap;

// primitive
#[test]
fn i32_from_pyobject() {
    Python::with_gil(|py| {
        let any: Py<PyAny> = 42.into_py(py);
        let i: i32 = from_pyobject(any.into_ref(py)).unwrap();
        assert_eq!(i, 42);
    });
}

#[test]
fn f32_from_pyobject() {
    Python::with_gil(|py| {
        let any: Py<PyAny> = (0.1).into_py(py);
        let x: f32 = from_pyobject(any.into_ref(py)).unwrap();
        assert_eq!(x, 0.1);
    });
}

#[test]
fn bool_from_pyobject() {
    Python::with_gil(|py| {
        let any: Py<PyAny> = true.into_py(py);
        let x: bool = from_pyobject(any.into_ref(py)).unwrap();
        assert_eq!(x, true);
    });
}

// option
#[test]
fn option_from_pyobject() {
    Python::with_gil(|py| {
        let none = py.None();
        let option: Option<i32> = from_pyobject(none.into_ref(py)).unwrap();
        assert_eq!(option, None);

        let py_int: Py<PyAny> = 42.into_py(py);
        let i: Option<i32> = from_pyobject(py_int.into_ref(py)).unwrap();
        assert_eq!(i, Some(42));
    })
}

// unit
#[test]
fn unit_from_pyobject() {
    Python::with_gil(|py| {
        let py_unit = PyTuple::empty(py);
        let unit: () = from_pyobject(py_unit).unwrap();
        assert_eq!(unit, ());
    })
}

// unit struct
#[derive(Debug, PartialEq, Deserialize)]
struct UnitStruct;

#[test]
fn unit_struct_from_pyobject() {
    Python::with_gil(|py| {
        let py_unit = PyTuple::empty(py);
        let unit: UnitStruct = from_pyobject(py_unit).unwrap();
        assert_eq!(unit, UnitStruct);
    })
}

// unit variant
#[derive(Debug, PartialEq, Deserialize)]
enum E {
    A,
    B,
}

#[test]
fn unit_variant_from_pyobject() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "E" => "A"
        }
        .unwrap();
        let out: E = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(out, E::A);
    })
}

// newtype struct
#[derive(Debug, PartialEq, Deserialize)]
struct NewTypeStruct(u8);

#[test]
fn newtype_struct_from_pyobject() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "NewTypeStruct" => 1
        }
        .unwrap();
        let obj: NewTypeStruct = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(obj, NewTypeStruct(1));
    });
}

// newtype variant
#[derive(Debug, PartialEq, Deserialize)]
enum NewTypeVariant {
    N(u8),
}

#[test]
fn newtype_variant_from_pyobject() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "NewTypeVariant" => ("N", 41)
        }
        .unwrap();
        let obj: NewTypeVariant = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(obj, NewTypeVariant::N(41));
    });
}

// seq
#[test]
fn seq_from_pyobject() {
    Python::with_gil(|py| {
        let list = PyList::new(py, &[1, 2, 3]);
        let seq: Vec<i32> = from_pyobject(list).unwrap();
        assert_eq!(seq, vec![1, 2, 3]);
    });
}

// tuple
#[test]
fn tuple_from_pyobject() {
    Python::with_gil(|py| {
        let tuple = PyTuple::new(py, &[1, 2, 3]);
        let tuple: (i32, i32, i32) = from_pyobject(tuple).unwrap();
        assert_eq!(tuple, (1, 2, 3));
    });
}

// tuple struct
#[derive(Debug, PartialEq, Deserialize)]
struct T(u8, String);

#[test]
fn tuple_struct_from_pyobject() {
    Python::with_gil(|py| {
        let dict = pydict! {
            py,
            "T" => (1, "test")
        }
        .unwrap();
        let obj: T = from_pyobject(dict).unwrap();
        assert_eq!(obj, T(1, "test".to_string()));
    });
}

// TODO tuple variant

// map
#[test]
fn btreemap_from_pydict() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "a" => "hom",
            "b" => "test"
        }
        .unwrap();
        let map: BTreeMap<String, String> = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(map.get("a"), Some(&"hom".to_string()));
        assert_eq!(map.get("b"), Some(&"test".to_string()));
    });
}

// struct
#[derive(Debug, PartialEq, Deserialize)]
struct A {
    a: i32,
    b: String,
}

#[test]
fn struct_from_pydict() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "a" => 1,
            "b" => "test"
        }
        .unwrap();
        let a: A = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(
            a,
            A {
                a: 1,
                b: "test".to_string()
            }
        );
    });
}

#[test]
fn struct_from_nested_pydict() {
    Python::with_gil(|py| {
        let dict = pydict! {
            "A" => pydict! {
                "a" => 1,
                "b" => "test"
            }
            .unwrap()
        }
        .unwrap();
        let a: A = from_pyobject(dict.into_ref(py)).unwrap();
        assert_eq!(
            a,
            A {
                a: 1,
                b: "test".to_string()
            }
        );
    });
}

// TODO struct variant
