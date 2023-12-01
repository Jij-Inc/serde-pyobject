use pyo3::prelude::*;
use serde::Serialize;

fn to_json_to_pyobject<'py, T: Serialize>(py: Python<'py>, obj: T) -> PyResult<&'py PyAny> {
    let json = serde_json::to_string(&obj).unwrap();
    let obj = py.import("json")?.getattr("loads")?.call1((json,))?;
    Ok(obj)
}

fn test(obj: impl Serialize) {
    Python::with_gil(|py| {
        let direct = serde_pyobject::to_pyobject(py, &obj).unwrap();
        let by_json = to_json_to_pyobject(py, obj).unwrap();
        assert!(dbg!(direct).eq(dbg!(by_json)).unwrap());
    })
}

#[test]
fn primitive() {
    test(1_u8);
    test(-4_i32);
    test(-3.1);
    test(true);
}

#[test]
fn option() {
    test(Some(10_u8));
    test(None::<u8>);
}

#[test]
fn unit() {
    test(());
}

#[derive(Serialize)]
struct UnitStruct;

#[test]
fn unit_struct() {
    test(UnitStruct);
}

#[derive(Serialize)]
enum UnitVariant {
    A,
    B,
}

#[test]
fn unit_variant() {
    test(UnitVariant::A);
    test(UnitVariant::B);
}

#[derive(Serialize)]
enum NewtypeVariant {
    N(u8),
}

#[test]
fn newtype_variant() {
    test(NewtypeVariant::N(10));
}

#[derive(Serialize)]
struct A {
    a: i32,
    b: String,
}

#[test]
fn struct_() {
    test(A {
        a: 10,
        b: "hello".to_owned(),
    });
}
