
use maplit::hashmap;
use pyo3::{ffi::c_str, prelude::*};
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

fn check_revertible<'de, T: Serialize + Deserialize<'de> + PartialEq + std::fmt::Debug>(obj: T) {
    Python::with_gil(|py| {
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

#[test]
fn check_python_object() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        name: String,
        age: i32,
    }

    Python::with_gil(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
class MyClass:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        "#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass("John", 30)
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            name: "John".to_string(),
            age: 30,
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}

#[cfg(feature = "pydantic_support")]
#[test]
fn check_pydantic_object() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        name: String,
        age: i32,
    }

    Python::with_gil(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
from pydantic import BaseModel
class MyClass(BaseModel):
    name: str
    age: int
"#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass(name="John", age=30)
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            name: "John".to_string(),
            age: 30,
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        println!("any: {:?}", any);

        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}

#[cfg(feature = "dataclass_support")]
#[test]
fn check_dataclass_object() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        name: String,
        age: i32,
    }

    Python::with_gil(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
from dataclasses import dataclass
@dataclass
class MyClass:
    name: str
    age: int
"#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass(name="John", age=30)
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            name: "John".to_string(),
            age: 30,
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        println!("any: {:?}", any);

        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}

#[cfg(feature = "dataclass_support")]
#[test]
fn check_dataclass_object_nested() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClassNested {
        name: String,
        age: i32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyClass {
        my_class: MyClassNested,
    }

    Python::with_gil(|py| {
        // Create an instance of Python object
        py.run(
            c_str!(
                r#"
from dataclasses import dataclass
@dataclass
class MyClassNested:
    name: str
    age: int

@dataclass
class MyClass:
    my_class: MyClassNested
"#
            ),
            None,
            None,
        )
        .unwrap();
        // Create an instance of MyClass
        let my_python_class = py
            .eval(
                c_str!(
                    r#"
MyClass(my_class=MyClassNested(name="John", age=30))
"#
                ),
                None,
                None,
            )
            .unwrap();

        let my_rust_class = MyClass {
            my_class: MyClassNested {
                name: "John".to_string(),
                age: 30,
            },
        };
        let any: Bound<'_, PyAny> = to_pyobject(py, &my_rust_class).unwrap();
        println!("any: {:?}", any);

        let rust_version: MyClass = from_pyobject(my_python_class).unwrap();
        let python_version: MyClass = from_pyobject(any).unwrap();
        assert_eq!(rust_version, python_version);
    })
}
