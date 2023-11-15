use pyo3::prelude::*;
use serde::Deserialize;
use serde_pyobject::{from_pyobject, pydict};
use std::collections::BTreeMap;

// primitive
#[test]
fn i32_from_pyobject() {
    Python::with_gil(|py| {
        let py_int: Py<PyAny> = 42.into_py(py);
        let i32: i32 = from_pyobject(py_int.into_ref(py)).unwrap();
        assert_eq!(i32, 42);
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

// TODO unit
// TODO unit struct
// TODO unit variant

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

// TODO newtype variant
// TODO seq
// TODO tuple
// TODO tuple struct
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
