use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::ast::{AtomExp, MeasureType, Concept};
use super::objects::obj::{ObjType, DATA, ATTR};
use crate::expdata::expdata::ExpData;
use super::expstructure::{ExpStructure, Parastructure, Objstructure, ExpConfig, DataStructOfExpData, DataStruct};

#[pymethods]
impl ObjType {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

#[pymethods]
impl Objstructure {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
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
    #[getter]#[inline]
    fn get_all_ids(&self) -> HashSet<i32> {
        self.get_ref_expconfig().obj_name_map.keys().cloned().collect()
    }
    #[inline]
    fn get_obj_type(&self, id: i32) -> ObjType {
        self.get_ref_expconfig().obj_name_map.get(&id).unwrap().0.clone()
    }
    #[inline]
    pub fn get_obj(&self, id: i32) -> Objstructure {
        self.get_ref_expconfig().get_obj(id).clone()
    }
    #[getter]#[inline]
    fn obj_info(&self) -> HashMap<String, (ObjType, i32)>{
        let expdata = self.get_ref_expconfig();
        expdata.obj_id_map.clone()
    }
    #[getter]#[inline]
    fn data_info(&self) -> DataStruct{
        if self.expdata_is_none() {
            panic!("The expdata has not been collected yet.");
        }
        let expdata = self.get_ref_expdata();
        // for (key, value) in expdata.data.iter() {
        //     println!("{}: {}", key.0, key.1);
        // }
        expdata.data.clone()
    }
    #[getter]
    fn spdim(&self) -> usize {
        self.get_ref_expconfig().spdim
    }
    fn random_settings(&mut self) {
        self.random_sample();
    }
    fn random_set_exp_para(&mut self) {
        self.random_sample_exp_para();
    }
    fn random_set_obj(&mut self, id: i32) {
        self.random_sample_obj(id);
    }
    pub fn collect_expdata(&mut self, measuretype: MeasureType) -> DataStructOfExpData {
        self.get_expdata(measuretype).clone()
    }
    fn copy(&self) -> Self {
        self.clone()
    }
}

#[pymethods]
impl DataStruct {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[getter]#[inline]
    fn get_data_keys(&self) -> HashSet<AtomExp> {
        self.get_data().keys().cloned().collect()
    }
    #[inline]
    fn fetch_data_by_name_ids(&self, name: &str, ids: Vec<i32>) -> Option<ExpData> {
        self.get_data().get(&AtomExp::new_variable_ids(name.to_string(), ids)).cloned()
    }
    #[inline]
    fn fetch_data_by_key(&self, atom: AtomExp) -> Option<ExpData> {
        self.get_data().get(&atom).cloned()
    }
    #[inline]
    fn fetch_data_by_str(&self, atom_name: &str) -> Option<ExpData> {
        self.get_data().get(&AtomExp::from_string(atom_name.to_string())).cloned()
    }
    #[staticmethod]
    fn empty() -> Self {
        DataStruct::new(HashMap::new())
    }
    fn add_data(&mut self, atom: AtomExp, expdata: ExpData) {
        self.set_data(atom, expdata);
    }
    fn remove_data(&mut self, atom: AtomExp) {
        self.reset_data(atom);
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
    fn __new__(name: &str, spdim: usize, exp_para: HashMap<String, Parastructure>,
               obj_info: HashMap<String, Objstructure>,
               data_info: Vec<(Concept, Vec<String>)>) -> Self {
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