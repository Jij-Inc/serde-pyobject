use pyo3::{types::*, Bound, PyResult, Python};

/// Check if the given object is an instance of a dataclass by `dataclasses.is_dataclass`,
/// and convert it to a PyDict using `dataclasses.asdict`.
pub fn dataclass_as_dict<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<Bound<'py, PyDict>>> {
    let module = PyModule::import(py, "dataclasses").expect("Failed to import dataclasses");
    let is_dataclass_fn = module
        .getattr("is_dataclass")
        .expect("Failed to get is_dataclass")
        .cast_into::<PyFunction>()
        .expect("Failed to cast to PyFunction");

    if is_dataclass_fn.call1((obj,))?.extract::<bool>()? {
        let asdict_fn = module
            .getattr("asdict")
            .expect("Failed to get asdict")
            .cast_into::<PyFunction>()
            .expect("Failed to cast to PyFunction");
        let dict_obj = asdict_fn.call1((obj,))?;
        let dict = dict_obj.cast_into::<PyDict>()?;
        Ok(Some(dict))
    } else {
        Ok(None)
    }
}
