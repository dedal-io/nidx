use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

create_exception!(_nidx, NidError, PyValueError);
create_exception!(_nidx, NidFormatError, NidError);
create_exception!(_nidx, NidChecksumError, NidError);
create_exception!(_nidx, NidInvalidDateError, NidError);

#[pyclass(frozen, name = "NidInfo")]
struct PyNidInfo {
    #[pyo3(get)]
    country: String,
    #[pyo3(get)]
    birthday: String,
    #[pyo3(get)]
    sex: String,
    #[pyo3(get)]
    is_national: bool,
    #[pyo3(get)]
    year: u16,
    #[pyo3(get)]
    month: u8,
    #[pyo3(get)]
    day: u8,
}

#[pymethods]
impl PyNidInfo {
    fn __repr__(&self) -> String {
        format!(
            "NidInfo(country='{}', birthday='{}', sex='{}', is_national={}, year={}, month={}, day={})",
            self.country,
            self.birthday,
            self.sex,
            if self.is_national { "True" } else { "False" },
            self.year,
            self.month,
            self.day,
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.country == other.country
            && self.birthday == other.birthday
            && self.sex == other.sex
            && self.is_national == other.is_national
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.country.hash(&mut hasher);
        self.birthday.hash(&mut hasher);
        self.sex.hash(&mut hasher);
        self.is_national.hash(&mut hasher);
        hasher.finish()
    }
}

fn albania_to_py_err(e: nidx::albania::NidError) -> PyErr {
    let msg = e.to_string();
    match e {
        nidx::albania::NidError::Format(_) => NidFormatError::new_err(msg),
        nidx::albania::NidError::Checksum => NidChecksumError::new_err(msg),
        nidx::albania::NidError::InvalidDate(_) => NidInvalidDateError::new_err(msg),
        _ => NidError::new_err(msg),
    }
}

/// Submodule for Kosovo personal number operations.
fn kosovo_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "kosovo")?;

    #[pyfunction]
    fn is_valid(nid: &str) -> bool {
        nidx::kosovo::is_valid(nid)
    }

    #[pyfunction]
    fn validate(nid: &str) -> PyResult<()> {
        nidx::kosovo::validate(nid).map_err(|e| {
            let msg = e.to_string();
            match e {
                nidx::kosovo::NidError::Format(_) => NidFormatError::new_err(msg),
                nidx::kosovo::NidError::Checksum => NidChecksumError::new_err(msg),
                _ => NidError::new_err(msg),
            }
        })
    }

    m.add_function(wrap_pyfunction!(is_valid, &m)?)?;
    m.add_function(wrap_pyfunction!(validate, &m)?)?;
    Ok(m)
}

/// Submodule for Albanian NID operations.
fn albania_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "albania")?;

    #[pyfunction]
    fn validate(nid: &str) -> PyResult<()> {
        nidx::albania::validate(nid).map_err(albania_to_py_err)
    }

    #[pyfunction]
    fn decode(nid: &str) -> PyResult<PyNidInfo> {
        let info = nidx::albania::decode(nid).map_err(albania_to_py_err)?;
        Ok(PyNidInfo {
            country: "albania".to_string(),
            birthday: info.birthday.to_string(),
            sex: info.sex.to_string(),
            is_national: info.is_national,
            year: info.birthday.year,
            month: info.birthday.month,
            day: info.birthday.day,
        })
    }

    #[pyfunction]
    fn is_valid(nid: &str) -> bool {
        nidx::albania::is_valid(nid)
    }

    m.add_function(wrap_pyfunction!(validate, &m)?)?;
    m.add_function(wrap_pyfunction!(decode, &m)?)?;
    m.add_function(wrap_pyfunction!(is_valid, &m)?)?;
    Ok(m)
}

#[pymodule]
fn _nidx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    let albania = albania_module(py)?;
    m.add_submodule(&albania)?;
    let kosovo = kosovo_module(py)?;
    m.add_submodule(&kosovo)?;

    m.add_class::<PyNidInfo>()?;
    m.add("NidError", py.get_type::<NidError>())?;
    m.add("NidFormatError", py.get_type::<NidFormatError>())?;
    m.add("NidChecksumError", py.get_type::<NidChecksumError>())?;
    m.add("NidInvalidDateError", py.get_type::<NidInvalidDateError>())?;
    Ok(())
}
