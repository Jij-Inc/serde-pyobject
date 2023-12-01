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
        assert!(direct.eq(by_json).unwrap());
    })
}

#[test]
fn primitive() {
    test(1_u8);
    test(-4_i32);
    test(-3.1);
    test(true);
}
