use pyo3::{types::*, Bound, PyResult, Python};

/// Check if the given object is an instance of pydantic BaseModel,
/// and convert it to a PyDict using `model_dump()`.
///
/// If pydantic is not installed, this function returns Ok(None).
pub fn pydantic_model_as_dict<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> PyResult<Option<Bound<'py, PyDict>>> {
    // Try to import pydantic module
    let module = match PyModule::import(py, "pydantic") {
        Ok(m) => m,
        Err(_) => {
            // If pydantic import fails for any reason, return None
            log::debug!("pydantic module not found; skipping pydantic model check");
            return Ok(None);
        }
    };

    let base_model = module.getattr("BaseModel")?.cast_into::<PyType>()?;

    if obj.is_instance(&base_model)? {
        let model_dump_fn = obj.getattr("model_dump")?;
        let dict_obj = model_dump_fn.call0()?;
        let dict = dict_obj.cast_into::<PyDict>()?;
        Ok(Some(dict))
    } else {
        Ok(None)
    }
}
