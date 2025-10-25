use pyo3::{types::*, Bound, PyResult, Python};

/// Check if the given object is an instance of a dataclass by `dataclasses.is_dataclass`,
/// and convert it to a PyDict using `dataclasses.asdict`.
pub fn dataclass_as_dict<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<Bound<'py, PyDict>>> {
    let module = PyModule::import(py, "dataclasses")?;
    let is_dataclass_fn = module.getattr("is_dataclass")?;

    if is_dataclass_fn.call1((obj,))?.extract::<bool>()? {
        let asdict_fn = module.getattr("asdict")?;
        let dict_obj = asdict_fn.call1((obj,))?;
        let dict = dict_obj.cast_into::<PyDict>()?;
        Ok(Some(dict))
    } else {
        Ok(None)
    }
}
