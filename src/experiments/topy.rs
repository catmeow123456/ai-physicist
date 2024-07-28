use pyo3::prelude::*;
use std::collections::HashMap;

use super::objects::obj::{ObjType, DATA, ATTR};
use super::expstructure::{ExpStructure, Parastructure, Objstructure, ExpConfig, DataStructOfExpData};

#[pymethods]
impl Objstructure {
    fn random_settings(&mut self) {
        self.random_sample();
    }
    #[staticmethod]
    fn make_masspoint(mass_low: f64, mass_high: f64) -> Self {
        Objstructure::masspoint((mass_low, mass_high))
    }
    #[staticmethod]
    fn make_spring(k_low: f64, k_high: f64, l_low: f64, l_high: f64) -> Self {
        Objstructure::spring((k_low, k_high), (l_low, l_high))
    }
}

#[pymethods]
impl DATA {
    #[new]
    fn __new__(obj: ObjType, name: &str) -> Self {
        DATA::Mk { obj, name: name.to_string() }
    }
}

#[pymethods]
impl ATTR {
    #[new]
    fn __new__(obj: ObjType, name: &str) -> Self {
        ATTR::Mk { obj, name: name.to_string() }
    }
}

#[pymethods]
impl ExpConfig {
    #[new]
    fn __new__(name: &str, spdim: usize, exp_para: HashMap<String, Parastructure>, obj_info: HashMap<String, Objstructure>, data_info: HashMap<String, Vec<DATA>>) -> Self {
        ExpConfig::new(name.to_string(), spdim, exp_para, obj_info, data_info)
    }
}

#[pymethods]
impl ExpStructure {
    fn random_settings(&mut self) {
        self.random_sample();
    }
    pub fn collect_expdata(&mut self, t_end: f64, t_num: usize, error: f64, repeat_time: usize) -> DataStructOfExpData {
        self.get_expdata(t_end, t_num, error, repeat_time).clone()
    }
}

#[pymodule]
pub fn register_experiment(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DATA>()?;
    m.add_class::<ATTR>()?;
    m.add_class::<Objstructure>()?;
    m.add_class::<Parastructure>()?;
    m.add_class::<ExpConfig>()?;
    m.add_class::<ExpStructure>()?;
    Ok(())
}