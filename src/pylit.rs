#[macro_export]
macro_rules! pydict {
    ($py:expr, $($key:expr => $value:expr),*) => {
        (|| -> $crate::pyo3::PyResult<& $crate::pyo3::types::PyDict> {
            let dict = $crate::pyo3::types::PyDict::new($py);
            $(dict.set_item($key, $value)?;)*
            Ok(dict)
        })()
    };
    ($($key:expr => $value:expr),*) => {
        $crate::pyo3::Python::with_gil(|py| -> $crate::pyo3::PyResult<$crate::pyo3::Py<$crate::pyo3::types::PyDict>> {
            let dict = pydict!(py, $($key => $value),*)?;
            Ok(dict.into())
        })
    };
}

#[macro_export]
macro_rules! pylist {
    ($py:expr, $($value:expr),*) => {
        (|| -> $crate::pyo3::PyResult<& $crate::pyo3::types::PyList> {
            let list = $crate::pyo3::types::PyList::new($py, vec![ $($value),* ]);
            Ok(list)
        })()
    };
    ($($value:expr),*) => {
        $crate::pyo3::Python::with_gil(|py| -> $crate::pyo3::PyResult<$crate::pyo3::Py<$crate::pyo3::types::PyList>> {
            let list = pylist!(py, $($value),*)?;
            Ok(list.into())
        })
    };
}

#[cfg(test)]
mod test {
    use pyo3::prelude::*;

    #[test]
    fn create_pylist() {
        Python::with_gil(|py| {
            let list = pylist![py, 1, 2, 3].unwrap();
            assert_eq!(list.len(), 3);
            assert_eq!(list.get_item(0).unwrap().extract::<i32>().unwrap(), 1);
            assert_eq!(list.get_item(1).unwrap().extract::<i32>().unwrap(), 2);
            assert_eq!(list.get_item(2).unwrap().extract::<i32>().unwrap(), 3);
        })
    }

    #[test]
    fn create_pydict() {
        Python::with_gil(|py| {
            let dict = pydict! {
                py,
                "foo" => 42,
                "bar" => "baz"
            }
            .unwrap();

            assert_eq!(
                dict.get_item("foo")
                    .unwrap()
                    .unwrap()
                    .extract::<i32>()
                    .unwrap(),
                42
            );
            assert_eq!(
                dict.get_item("bar")
                    .unwrap()
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
                "baz",
            );
        })
    }
}
