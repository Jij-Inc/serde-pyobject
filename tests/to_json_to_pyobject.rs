use maplit::*;
use pyo3::prelude::*;
use serde::Serialize;

fn to_json_to_pyobject<T: Serialize>(py: Python<'_>, obj: T) -> PyResult<Bound<'_, PyAny>> {
    let json = serde_json::to_string(&obj).unwrap();
    let obj = py.import("json")?.getattr("loads")?.call1((json,))?;
    Ok(obj)
}

fn test(obj: impl Serialize) {
    Python::attach(|py| {
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
fn string() {
    test("test");
    test("test".to_string());
}

#[test]
fn option() {
    test(Some(10_u8));
    test(None::<u8>);
}

// skip unit
//
// Input: `()`
// Output:
// - to_pyobject = `()`
// - to_json_to_pyobject = `None`

// skip unit_struct
//
// #[derive(Serialize)]
// struct UnitStruct;
//
// Input: `UnitStruct`
// - to_pyobject = `()`
// - to_json_to_pyobject = `None`

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
struct NewtypeStruct(u8);

#[test]
fn newtype_struct() {
    test(NewtypeStruct(10));
}

#[derive(Serialize)]
enum NewtypeVariant {
    N(u8),
}

#[test]
fn newtype_variant() {
    test(NewtypeVariant::N(10));
}

#[test]
fn seq() {
    test(vec![1_u8, 2, 3]);
}

// Skip tuple
//
// Input: `(1, "test")`
// Output:
// - to_pyobject = `(1, "test")`
// - to_json_to_pyobject = `[1, "test"]`

// Skip tuple_struct
//
// #[derive(Serialize)]
// struct TupleStruct(u8, u8, u8);
//
// Input: `TupleStruct(1, 2, 3)`
// Output:
// - to_pyobject = `(1, 2, 3)`
// - to_json_to_pyobject = `[1, 2, 3]`

// Skip tuple_variant
//
// #[derive(Serialize)]
// enum TupleVariant {
//     T(u8, u8),
// }
//
// Input: `TupleVariant::T(1, 2)`
// Output:
// - to_pyobject = {'T': (1, 2)}
// - to_json_to_pyobject = {'T': [1, 2]}

#[test]
fn map() {
    test(hashmap! {
        "a".to_owned() => 1_u8,
        "b".to_owned() => 2,
        "c".to_owned() => 3,
    });
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

#[derive(Serialize)]
enum StructVariant {
    S { r: u8, g: u8, b: u8 },
}

#[test]
fn struct_variant() {
    test(StructVariant::S {
        r: 10,
        g: 20,
        b: 30,
    });
}
