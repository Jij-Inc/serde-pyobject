use pyo3::prelude::*;
use serde::Deserialize;
use serde_pyobject::{from_pyobject, pydict};
use std::collections::BTreeMap;

#[test]
fn i32_from_pyobject() {
    Python::with_gil(|py| {
        let py_int: Py<PyAny> = 42.into_py(py);
        let i32: i32 = from_pyobject(py_int.into_ref(py)).unwrap();
        assert_eq!(i32, 42);
    });
}

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
