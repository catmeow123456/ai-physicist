use pyo3::prelude::*;
use crate::ast::{AtomExp, Exp, MeasureType, TExp};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{fmt, collections::HashMap};
use ndarray::{ArrayBase, Array1, Array2, Dimension, OwnedRepr};
use super::expdata::ExpData;
use super::objects::obj::{ObjType, ATTR};
use super::objects::obj::DATA;

pub type DoExpType = fn(f64,usize,f64,&ExpConfig) -> DataStructOfDoExperiment;

// 刻画某个参数结构的抽象类
// range: 参数的取值范围
#[pyclass]
#[derive(Clone)]
pub struct Parastructure {
    value: Option<f64>,
    range: (f64, f64),
}

impl Parastructure {
    pub fn new(range: Option<(f64, f64)>) -> Self {
        Parastructure {
            value: None,
            range: range.unwrap_or((-1e10, 1e10)),
        }
    }
    // fn set_value(&mut self, value: f64) {
    //     self.value = Some(value);
    // }
    fn random_sample(&mut self) {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(self.range.0..self.range.1);
        self.value = Some(value);
    }
    fn real_value(&self) -> f64 {
        self.value.unwrap()
    }
}

impl fmt::Display for Parastructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Parastructure] value: {:?}, range: {:?}",
               self.value, self.range).unwrap();
        Result::Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Objstructure {
    pub obj_type: ObjType,
    attribute: HashMap<ATTR, Parastructure>,
}
impl Objstructure {
    pub fn new(obj_type: ObjType,
           attribute: HashMap<ATTR, Parastructure>) -> Self {
        Objstructure {
            obj_type,
            attribute,
        }
    }
    pub fn random_sample(&mut self) {
        for (_, para) in self.attribute.iter_mut() {
            para.random_sample();
        }
    }
    // fn real_value(&self) -> HashMap<ATTR, f64> {
    //     let mut result = HashMap::new();
    //     for (name, para) in self.attribute.iter() {
    //         result.insert(name.clone(), para.real_value());
    //     }
    //     result
    // }
    pub fn get_para_real_value(&self, para_name: &ATTR) -> f64 {
        self.attribute.get(para_name).unwrap().real_value()
    }
    // fn set_value(&mut self, value_dict: HashMap<ATTR, f64>) {
    //     for (name, value) in value_dict.iter() {
    //         self.attribute.get_mut(name).unwrap().set_value(*value);
    //     }
    // }
}

impl fmt::Display for Objstructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Objstructure] obj_type: {}, attribute: {:?}",
               self.obj_type, self.attribute.keys()).unwrap();
        write!(f, "\nAttribute:").unwrap();
        for (name, para) in self.attribute.iter() {
            write!(f, "\n| {}: {}", name, para).unwrap();
        }
        write!(f, ".").unwrap();
        Result::Ok(())
    }
}

#[pyclass]
pub struct DataStructOfDoExperiment {
    n: usize,
    obj_id_map: HashMap<String, (ObjType, i32)>,
    data: HashMap<AtomExp, Array1<f64>>,
}
impl DataStructOfDoExperiment {
    fn new(n: usize,
           obj_id_map: HashMap<String, (ObjType, i32)>,
        ) -> Self {
        DataStructOfDoExperiment {
            n,
            obj_id_map,
            data: HashMap::new(),
        }
    }
    pub fn add_data(&mut self, key: (TExp, Vec<String>), data: &Array1<f64>) {
        assert_eq!(data.len(), self.n);
        let mut obj_ids = vec![];
        for obj_name in key.1.iter() {
            obj_ids.push(self.obj_id_map.get(obj_name).unwrap().1);
        }
        self.data.insert(key.0.to_atomexp(obj_ids), data.clone());
    }
    fn get_data(&self) -> &HashMap<AtomExp, Array1<f64>> {
        &self.data
    }
}

#[pymethods]
impl MeasureType {
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn repeat_time(&self) -> usize {
        self.repeat_time
    }
    pub fn error(&self) -> f64 {
        self.error
    }
    pub fn t_end(&self) -> f64 {
        self.t_end
    }
    #[staticmethod]
    pub fn default() -> Self {
        MeasureType {
            n: 100,
            repeat_time: 100,
            error: 1e-8,
            t_end: 2.0,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct DataStruct {
    data: HashMap<AtomExp, ExpData>,
}
impl DataStruct {
    pub fn new(data: HashMap<AtomExp, ExpData>) -> Self {
        DataStruct {
            data
        }
    }
    pub fn set_data(&mut self, atom: AtomExp, expdata: ExpData) {
        self.data.insert(atom, expdata);
    }
    pub fn reset_data(&mut self, atom: AtomExp) {
        self.data.remove(&atom);
    }
    pub fn get_data(&self) -> &HashMap<AtomExp, ExpData> {
        &self.data
    }
    pub fn get_data_by_key(&self, atom: AtomExp) -> Result<&ExpData, String> {
        match self.data.get(&atom) {
            Some(value) => Ok(value),
            None => Err(format!("Data {} not found", atom)),
        }
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<AtomExp, ExpData> {
        self.data.iter()
    }
}

impl fmt::Display for DataStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DataStruct] data:").unwrap();
        for key in self.data.keys() {
            write!(f, "{},", key)?;
        }
        write!(f, ".")
    }
}

#[pyclass]
#[derive(Clone)]
pub struct DataStructOfExpData {
    pub name: String,
    pub measuretype: MeasureType,
    pub data: DataStruct,
}
impl DataStructOfExpData {
    fn new(name: String, measuretype: MeasureType, data: DataStruct) -> Self {
        DataStructOfExpData {
            name,
            measuretype,
            data
        }
    }
    pub fn set_data(&mut self, data: DATA, id: i32, expdata: ExpData) {
        self.data.set_data(data, id, expdata);
    }
    pub fn get_data(&self) -> &DataStruct {
        &self.data
    }
    pub fn get_t(&self) -> &ExpData {
        self.data.get_data_by_key(DATA::time(), 0).unwrap()
    }
    pub fn get_data_by_name_id(&self, name: &str, id: i32) -> Result<&ExpData, String> {
        self.data.get_data_by_name_id(name, id)
    }
    pub fn plot_expdata(&self, name: &str) {
        // plot the arr
        let mut plot = plotly::Plot::new();
        let t = self.get_t();
        let repeat_time = t.repeat_time;
        for ith in 0..repeat_time {
            let t= t.data.row(ith).to_vec();
            for (key, value) in self.data.iter() {
                if key.0 == DATA::time() {
                    continue;
                }
                let x = value.data.row(ith).to_vec();
                let trace = plotly::Scatter::new(t.clone(), x.clone());
                plot.add_trace(trace);
            }
        }
        // plot.show();
        plot.write_html(format!("tmp/{}.html", name));
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ExpConfig {
    // provided
    name: String,
    pub spdim: usize,
    exp_para: HashMap<String, Parastructure>,
    obj_info: HashMap<String, Objstructure>,
    data_info: Vec<(TExp, Vec<String>)>,
    // auto generated
    pub obj_id_map: HashMap<String, (ObjType, i32)>,
    obj_name_map: HashMap<i32, (ObjType, String)>,
    obj_info_dict: HashMap<ObjType, HashMap<i32, String>>,
}
impl ExpConfig {
    pub fn new(name: String, spdim: usize,
           exp_para: HashMap<String, Parastructure>,
           obj_info: HashMap<String, Objstructure>,
           data_info: Vec<(TExp, Vec<String>)>) -> Self {
        let mut obj_id_map: HashMap<String, (ObjType, i32)> = HashMap::new();
        let mut obj_info_dict: HashMap<ObjType, HashMap<i32, String>> = HashMap::new();
        let mut obj_name_map: HashMap<i32, (ObjType, String)> = HashMap::new();
        for (name, obj) in obj_info.iter() {
            if obj.obj_type == ObjType::Clock {
                obj_id_map.insert(name.clone(), (ObjType::Clock, 0));
                obj_name_map.insert(0, (ObjType::Clock, name.clone()));
            }
        }
        let mut hash_vec: Vec<(&String, &Objstructure)> = obj_info.iter().collect();
        hash_vec.sort_by(|a, b| (a.0).cmp(b.0));
        for (name, obj) in hash_vec.iter() {
            let obj_type = obj.obj_type.clone();
            if obj_type == ObjType::Clock {
                continue;
            }
            let obj_id = obj_id_map.len() as i32;
            obj_id_map.insert((*name).clone(), (obj_type.clone(), obj_id));
            obj_name_map.insert(obj_id, (obj_type.clone(), (*name).clone()));
            if !obj_info_dict.contains_key(&obj.obj_type) {
                obj_info_dict.insert(obj_type, HashMap::new());
            }
            obj_info_dict.get_mut(&obj.obj_type).unwrap().insert(obj_id, (*name).clone());
        }
        ExpConfig {
            name,
            spdim,
            exp_para,
            obj_info,
            data_info,
            obj_id_map,
            obj_name_map,
            obj_info_dict
        }
    }
    pub fn para(&self, para_name: &str) -> f64 {
        self.exp_para.get(para_name).unwrap().real_value()
    }
    pub fn obj(&self, obj_name: &str) -> &Objstructure {
        self.obj_info.get(obj_name).unwrap()
    }
    pub fn obj_para(&self, obj_name: &str, para_name: &ATTR) -> f64 {
        self.obj_info.get(obj_name).unwrap().get_para_real_value(para_name)
    }
    pub fn print_obj_info(&self) {
        println!("Name: {}; Object Info:", self.name);
        for (key, obj) in self.obj_info.iter() {
            println!("{}: {}", key, obj);
        }
    }
    fn get_obj(&self, id: i32) -> &Objstructure {
        let name = &self.obj_name_map.get(&id).unwrap().1;
        self.obj_info.get(name).unwrap()
    }
    fn set_obj(&mut self, id: i32, obj: Objstructure) {
        let name = self.obj_info_dict.get(&obj.obj_type).unwrap().get(&id).unwrap();
        self.obj_info.insert(name.clone(), obj);
    }
    fn random_sample(&mut self) {
        for (_, para) in self.exp_para.iter_mut() {
            para.random_sample();
        }
        for (_, obj) in self.obj_info.iter_mut() {
            obj.random_sample();
        }
    }
    pub fn create_data_struct_of_do_experiment(&self, t_num: usize) -> DataStructOfDoExperiment {
        for (data_texp, obj_names) in self.data_info.iter() {
            let mut texp_temp = data_texp;
            for obj_name in obj_names {
                let obj_type = self.obj_id_map.get(obj_name).unwrap().clone().0;
                match texp_temp {
                    TExp::Mksucc { objtype, texp, id:_ } => {
                        assert_eq!(*objtype, obj_type.to_string());
                        texp_temp = texp;
                    }
                    _ => panic!("DataStructOfDoExperiment: TExp not match, the data info dict has a wrong format."),
                }
            }
        }
        DataStructOfDoExperiment::new(
            t_num,
            self.obj_id_map.clone()
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ExpStructure {
    exp_config: ExpConfig,
    do_experiment: DoExpType,
    datastructofdata: Option<DataStructOfExpData>,
}
impl ExpStructure {
    pub fn new(exp_config: ExpConfig, do_experiment: DoExpType) -> Self {
        ExpStructure {
            exp_config,
            do_experiment,
            datastructofdata: None,
        }
    }
    pub fn name(&self) -> &str {
        &self.exp_config.name
    }
    pub fn print_obj_info(&self) {
        self.exp_config.print_obj_info();
    }
    pub fn get_obj(&self, id: i32) -> &Objstructure {
        self.exp_config.get_obj(id)
    }
    pub fn get_obj_ids(&self, obj_type: ObjType) -> Vec<i32> {
        self.exp_config.obj_info_dict.get(&obj_type).unwrap().keys().cloned().collect()
    }
    pub fn set_obj(&mut self, id: i32, obj: Objstructure) {
        if self.datastructofdata.is_some() {
            self.datastructofdata = None;
        }
        self.exp_config.set_obj(id, obj);
    }
    pub fn random_sample(&mut self) {
        if self.datastructofdata.is_some() {
            self.datastructofdata = None;
        }
        self.exp_config.random_sample();
    }
    pub fn calc_expdata(&mut self, measuretype: MeasureType) {
        let doexp = self.do_experiment;
        let t_end = measuretype.t_end();
        let t_num = measuretype.n();
        let repeat_time = measuretype.repeat_time();
        let error = measuretype.error();
        let data_struct = doexp(t_end, t_num, 0.0, &self.exp_config);
        let data = data_struct.get_data();
        let mut multi_data: HashMap<(DATA, i32), ExpData> = HashMap::new();
        for (name, data) in data.iter() {
            let mut idata: Array2<f64> = Array2::zeros((repeat_time, t_num));
            for i in 0..repeat_time {
                idata.row_mut(i).assign(&data);
            }
            idata = add_errors(&idata, error);
            assert_eq!(idata.shape(), [repeat_time, t_num]);
            multi_data.insert(name.clone(), ExpData::new(idata));
        }
        self.datastructofdata = Some(DataStructOfExpData::new(
            self.exp_config.name.clone(), measuretype,
            DataStruct::new(multi_data))
        );
    }
    pub fn get_expdata(&mut self, measuretype: MeasureType) -> &DataStructOfExpData {
        match self.datastructofdata.as_ref() {
            None => self.calc_expdata(measuretype),
            Some(datastructofdata) => {
                if datastructofdata.measuretype != measuretype {
                    self.calc_expdata(measuretype);
                }
            }
        }
        self.get_ref_expdata()
    }
    pub fn expdata_is_none(&self) -> bool {
        self.datastructofdata.is_none()
    }
    pub fn get_ref_expdata(&self) -> &DataStructOfExpData {
        self.datastructofdata.as_ref().unwrap()
    }
    pub fn get_mut_expdata(&mut self) -> &mut DataStructOfExpData {
        self.datastructofdata.as_mut().unwrap()
    }
    pub fn get_ref_expconfig(&self) -> &ExpConfig {
        &self.exp_config
    }
}

pub fn add_errors<D: Dimension>(array: &ArrayBase<OwnedRepr<f64>, D>, error: f64)
        -> ArrayBase<OwnedRepr<f64>, D> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, error).unwrap();
    array.mapv(|x| normal.sample(&mut rng) + x)
}
