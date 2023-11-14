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

#[cfg(test)]
mod test {
    use pyo3::prelude::*;

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
