use once_cell::sync::OnceCell;
use pyo3::{types::*, Bound, IntoPyObject, Py, PyResult, Python};

// Individual OnceCell instances for each cached item
#[cfg(feature = "dataclass_support")]
static DATACLASSES_MODULE: OnceCell<pyo3::PyObject> = OnceCell::new();
#[cfg(feature = "dataclass_support")]
static IS_DATACLASS_FN: OnceCell<pyo3::PyObject> = OnceCell::new();

#[cfg(feature = "pydantic_support")]
static PYDANTIC_MODULE: OnceCell<pyo3::PyObject> = OnceCell::new();
#[cfg(feature = "pydantic_support")]
static PYDANTIC_BASE_MODEL: OnceCell<pyo3::PyObject> = OnceCell::new();

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

#[cfg(feature = "dataclass_support")]
pub fn is_dataclass(py: Python, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
    // Initialize the dataclasses module if needed
    if DATACLASSES_MODULE.get().is_none() {
        let dataclasses = PyModule::import(py, "dataclasses")?;
        let _ = DATACLASSES_MODULE.set(dataclasses.into());
    }

    // Initialize the is_dataclass function if needed
    let is_dataclass_fn = if let Some(fn_obj) = IS_DATACLASS_FN.get() {
        fn_obj
    } else {
        let dataclasses = DATACLASSES_MODULE
            .get()
            .ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Dataclasses module not initialized")
            })?
            .bind(py);
        let is_dataclass_fn: Py<PyAny> = dataclasses
            .getattr("is_dataclass")?
            .into_pyobject(py)?
            .into();
        // Safe to unwrap because we know the value is set
        let _ = IS_DATACLASS_FN.set(is_dataclass_fn);
        IS_DATACLASS_FN.get().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Failed to initialize is_dataclass function")
        })?
    };

    // Execute the function
    let result = is_dataclass_fn.bind(py).call1((obj,))?;
    result.extract()
}
#[cfg(feature = "pydantic_support")]
pub fn is_pydantic_base_model(py: Python, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
    // First check if pydantic is installed
    if !is_module_installed(py, "pydantic")? {
        return Ok(false);
    }

    // Initialize pydantic module if needed
    if PYDANTIC_MODULE.get().is_none() {
        let pydantic = PyModule::import(py, "pydantic")?;
        // Safe to unwrap because we know the value is empty
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
        // Safe to unwrap because we know the value is empty
        let _ = PYDANTIC_BASE_MODEL.set(base_model);
        PYDANTIC_BASE_MODEL.get().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Failed to initialize BaseModel")
        })?
    };

    // Check if object is instance of BaseModel
    obj.is_instance(base_model.bind(py))
}
