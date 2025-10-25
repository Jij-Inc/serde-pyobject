//! Roundtrip Conversion Tests (Rust → Python → Rust)
//!
//! This test suite verifies that Rust values can be converted to Python and back without
//! data loss. Each test performs the following one-way roundtrip:
//!
//! 1. **Start**: Create a Rust value
//! 2. **Serialize**: Rust value → Python object (via `to_pyobject`)
//! 3. **Deserialize**: Python object → Rust value (via `from_pyobject`)
//! 4. **Assert**: The deserialized Rust value equals the original Rust value
//!
//! **Note**: These tests do NOT verify the reverse direction (Python → Rust → Python).
//! For tests that start with Python objects, see `python_types.rs`.
//!
//! This ensures that Rust data structures can safely cross the FFI boundary and return
//! to Rust without corruption, which is essential for round-trip serialization scenarios.
//!
//! Coverage includes:
//! - Primitive types (integers, floats, booleans, strings)
//! - Collections (vectors, maps)
//! - Structured data (structs, tuples, newtypes)
//! - Enums (unit, newtype, tuple, and struct variants)
//! - Option types
//!
//! For tests specific to Python types (dataclasses, Pydantic models, custom classes),
//! see `python_types.rs`.

use maplit::hashmap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

fn check_revertible<'de, T: Serialize + Deserialize<'de> + PartialEq + std::fmt::Debug>(obj: T) {
    Python::attach(|py| {
        let any = to_pyobject(py, &obj).unwrap();
        let reverted = from_pyobject(any).unwrap();
        assert_eq!(obj, reverted);
    })
}

#[test]
fn primitive() {
    check_revertible(1_u8);
    check_revertible(-4_i32);
    check_revertible(-3.1);
    check_revertible(true);
    check_revertible("test".to_string());
}

#[test]
fn option() {
    check_revertible(Some(10_u8));
    check_revertible(None::<u8>);
}

#[test]
fn unit() {
    check_revertible(());
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UnitStruct;

#[test]
fn unit_struct() {
    check_revertible(UnitStruct);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum UnitVariant {
    A,
    B,
}

#[test]
fn unit_variant() {
    check_revertible(UnitVariant::A);
    check_revertible(UnitVariant::B);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NewtypeStruct(u8);

#[test]
fn newtype_struct() {
    check_revertible(NewtypeStruct(10));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum NewtypeVariant {
    N(u8),
}

#[test]
fn newtype_variant() {
    check_revertible(NewtypeVariant::N(10));
}

#[test]
fn seq() {
    check_revertible(vec![1_u8, 2, 3]);
}

#[test]
fn tuple() {
    check_revertible((1, "test".to_string()));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TupleStruct(u8, u8, u8);

#[test]
fn tuple_struct() {
    check_revertible(TupleStruct(1, 2, 3));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TupleVariant {
    T(u8, u8),
}

#[test]
fn tuple_variant() {
    check_revertible(TupleVariant::T(1, 2));
}

#[test]
fn map() {
    check_revertible(hashmap! {
        "a".to_owned() => 1_u8,
        "b".to_owned() => 2,
        "c".to_owned() => 3,
    });
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct A {
    a: i32,
    b: String,
}

#[test]
fn struct_() {
    check_revertible(A {
        a: 10,
        b: "hello".to_owned(),
    });
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum StructVariant {
    S { r: u8, g: u8, b: u8 },
}

#[test]
fn struct_variant() {
    check_revertible(StructVariant::S {
        r: 10,
        g: 20,
        b: 30,
    });
}
