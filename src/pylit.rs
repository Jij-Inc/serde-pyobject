/// Create [`pyo3::types::PyDict`] from a list of key-value pairs.
///
/// Examples
/// ---------
///
/// - When you have GIL marker `py`, you can pass it and get a Bound pointer `PyResult<Bound<PyDict>>`:
///
/// ```
/// use pyo3::{Python, Bound, types::{PyDict, PyDictMethods, PyAnyMethods}};
/// use serde_pyobject::pydict;
///
/// Python::attach(|py| {
///     let dict: Bound<PyDict> = pydict! {
///         py,
///         "foo" => 42,
///         "bar" => "baz"
///     }
///     .unwrap();
///
///     assert_eq!(
///         dict.get_item("foo")
///             .unwrap()
///             .unwrap()
///             .extract::<i32>()
///             .unwrap(),
///         42
///     );
///     assert_eq!(
///         dict.get_item("bar")
///             .unwrap()
///             .unwrap()
///             .extract::<String>()
///             .unwrap(),
///         "baz",
///     );
/// })
/// ```
///
/// - When you don't have GIL marker, you get a `PyResult<Py<PyDict>>`:
///
/// ```
/// use pyo3::{Python, Py, types::{PyDict, PyDictMethods, PyAnyMethods}};
/// use serde_pyobject::pydict;
///
/// let dict: Py<PyDict> = pydict! {
///     "foo" => 42,
///     "bar" => "baz"
/// }
/// .unwrap();
///
/// Python::attach(|py| {
///     let dict = dict.into_bound(py);
///     assert_eq!(
///         dict.get_item("foo")
///             .unwrap()
///             .unwrap()
///             .extract::<i32>()
///             .unwrap(),
///         42
///     );
///     assert_eq!(
///         dict.get_item("bar")
///             .unwrap()
///             .unwrap()
///             .extract::<String>()
///             .unwrap(),
///         "baz",
///     );
/// })
/// ```
///
#[macro_export]
macro_rules! pydict {
    ($py:expr, $($key:expr => $value:expr),*) => {
        (|| -> $crate::pyo3::PyResult<$crate::pyo3::Bound<$crate::pyo3::types::PyDict>> {
            use $crate::pyo3::types::PyDictMethods;
            let dict = $crate::pyo3::types::PyDict::new_bound($py);
            $(dict.set_item($key, $value)?;)*
            Ok(dict)
        })()
    };
    ($($key:expr => $value:expr),*) => {
        $crate::pyo3::Python::attach(|py| -> $crate::pyo3::PyResult<$crate::pyo3::Py<$crate::pyo3::types::PyDict>> {
            let dict = pydict!(py, $($key => $value),*)?;
            Ok(dict.into())
        })
    };
}

/// Create [`pyo3::types::PyList`] from a list of values.
///
/// Examples
/// --------
///
/// - When you have GIL marker `py`, you can pass it and get a reference `PyResult<&PyList>`:
///
/// ```
/// use pyo3::{Python, types::{PyList, PyListMethods, PyAnyMethods}};
/// use serde_pyobject::pylist;
///
/// Python::attach(|py| {
///     let list = pylist![py; 1, "two"].unwrap();
///     assert_eq!(list.len(), 2);
///     assert_eq!(list.get_item(0).unwrap().extract::<i32>().unwrap(), 1);
///     assert_eq!(list.get_item(1).unwrap().extract::<String>().unwrap(), "two");
/// })
/// ```
///
/// - When you don't have GIL marker, you get a `PyResult<Py<PyList>>`:
///
/// ```
/// use pyo3::{Python, Py, types::{PyList, PyListMethods, PyAnyMethods}};
/// use serde_pyobject::pylist;
///
/// let list: Py<PyList> = pylist![1, "two"].unwrap();
///
/// Python::attach(|py| {
///    let list = list.into_bound(py);
///    assert_eq!(list.len(), 2);
///    assert_eq!(list.get_item(0).unwrap().extract::<i32>().unwrap(), 1);
///    assert_eq!(list.get_item(1).unwrap().extract::<String>().unwrap(), "two");
/// });
/// ```
///
#[macro_export]
macro_rules! pylist {
    ($py:expr; $($value:expr),*) => {
        (|| -> $crate::pyo3::PyResult<$crate::pyo3::Bound<$crate::pyo3::types::PyList>> {
            use $crate::pyo3::types::PyListMethods;
            let list = $crate::pyo3::types::PyList::empty_bound($py);
            $(list.append($value)?;)*
            Ok(list)
        })()
    };
    ($($value:expr),*) => {
        $crate::pyo3::Python::attach(|py| -> $crate::pyo3::PyResult<$crate::pyo3::Py<$crate::pyo3::types::PyList>> {
            let list = pylist!(py; $($value),*)?;
            Ok(list.into())
        })
    };
}
