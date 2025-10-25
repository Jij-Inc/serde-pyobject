use once_cell::sync::OnceCell;
use pyo3::{types::*, Bound, IntoPyObject, Py, PyResult, Python};

// Individual OnceCell instances for each cached item
static PYDANTIC_MODULE: OnceCell<Py<PyAny>> = OnceCell::new();
static PYDANTIC_BASE_MODEL: OnceCell<Py<PyAny>> = OnceCell::new();

fn is_module_installed(py: Python, module_name: &str) -> PyResult<bool> {
    match PyModule::import(py, module_name) {
        Ok(_) => Ok(true),
        Err(err) => {
            if err.is_instance_of::<pyo3::exceptions::PyModuleNotFoundError>(py) {
                Ok(false)
            } else {
                Err(err)
            }
        }
    }
}

pub fn is_pydantic_base_model(py: Python, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
    // First check if pydantic is installed
    if !is_module_installed(py, "pydantic")? {
        return Ok(false);
    }

    // Initialize pydantic module if needed
    if PYDANTIC_MODULE.get().is_none() {
        let pydantic = PyModule::import(py, "pydantic")?;
        // Safe to call set because we have checked the OnceCell is empty
        let _ = PYDANTIC_MODULE.set(pydantic.into());
    }

    // Initialize BaseModel if needed
    let base_model = if let Some(model) = PYDANTIC_BASE_MODEL.get() {
        model
    } else {
        let pydantic = PYDANTIC_MODULE
            .get()
            .ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Pydantic module not initialized")
            })?
            .bind(py);
        let base_model: Py<PyAny> = pydantic.getattr("BaseModel")?.into_pyobject(py)?.into();
        // Safe to call set because we have checked the OnceCell is empty
        let _ = PYDANTIC_BASE_MODEL.set(base_model);
        PYDANTIC_BASE_MODEL.get().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Failed to initialize BaseModel")
        })?
    };

    // Check if object is instance of BaseModel
    obj.is_instance(base_model.bind(py))
}
