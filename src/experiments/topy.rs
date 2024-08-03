use pyo3::prelude::*;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::ast::MeasureType;
use super::objects::obj::{ObjType, DATA, ATTR};
use super::expstructure::{ExpStructure, Parastructure, Objstructure, ExpConfig, DataStructOfExpData, DataStruct};

#[pymethods]
impl ObjType {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

#[pymethods]
impl Objstructure {
    fn random_settings(&mut self) {
        self.random_sample();
    }
    #[staticmethod]
    fn make_masspoint(m_low: f64, m_high: f64) -> Self {
        Objstructure::masspoint((m_low, m_high))
    }
    #[staticmethod]
    fn make_spring(k_low: f64, k_high: f64, l_low: f64, l_high: f64) -> Self {
        Objstructure::spring((k_low, k_high), (l_low, l_high))
    }
}

#[pymethods]
impl ExpStructure {
    fn obj_info(&self) -> HashMap<String, (ObjType, i32)>{
        let expdata = self.get_ref_expconfig();
        expdata.obj_id_map.clone()
    }
    fn data_info(&self) -> DataStruct{
        let expdata = self.get_ref_expdata();
        // for (key, value) in expdata.data.iter() {
        //     println!("{}: {}", key.0, key.1);
        // }
        expdata.data.clone()
    }
    fn spdim(&self) -> usize {
        self.get_ref_expconfig().spdim
    }
    fn random_settings(&mut self) {
        self.random_sample();
    }
    pub fn collect_expdata(&mut self, measuretype: MeasureType) -> DataStructOfExpData {
        self.get_expdata(measuretype).clone()
    }
}

#[pymethods]
impl DATA {
    #[new]
    fn __new__(obj: ObjType, name: &str) -> Self {
        DATA::Mk { obj, name: name.to_string() }
    }
    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    fn __str__(&self) -> String {
        format!("{}", self)
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

#[pymodule]
pub fn register_experiment(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DATA>()?;
    m.add_class::<ATTR>()?;
    m.add_class::<Objstructure>()?;
    m.add_class::<Parastructure>()?;
    m.add_class::<ExpConfig>()?;
    m.add_class::<ExpStructure>()?;
    m.add_class::<DataStruct>()?;
    m.add_class::<DataStructOfExpData>()?;
    Ok(())
}