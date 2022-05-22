use pyo3::{exceptions, prelude::*};
use xunmi::{self as x};

#[pyclass]
pub struct Indexer{x::Indexer};

#[pyclass]
pub struct InputConfig{x::InputConfig};

#[pyclass]
pub struct IndexUpdater{x::IndexUpdater};

#[pymodule]
fn xunmi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Indexer>()?;
    m.add_class::<InputConfig>()?;
    m.add_class::<IndexUpdater>()?;
    Ok(())
}

#[pymethods]
impl IndexUpdater {
    pub fn add(&mut self, input: &str, config: &InputConfig) -> PyResult<()> {
        Ok(self.0.add(input, &config.0).map_err(to_pyerr)?)
    }

    pub fn update(&mut self, input: &str, config: &InputConfig) -> PyResult<()> {
        Ok(self.0.update(input, &config.0).map_err(to_pyerr)?)
    }

    pub fn commit(&mut self) -> PyResult<()> {
        Ok(self.0.commit().map_err(to_pyerr)?)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        Ok(self.0.clear().map_err(to_pyerr)?)
    }
}