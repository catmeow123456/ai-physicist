use pyo3::prelude::*;
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
    fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }
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
    obj_type: ObjType,
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
    fn random_sample(&mut self) {
        for (_, para) in self.attribute.iter_mut() {
            para.random_sample();
        }
    }
    fn real_value(&self) -> HashMap<ATTR, f64> {
        let mut result = HashMap::new();
        for (name, para) in self.attribute.iter() {
            result.insert(name.clone(), para.real_value());
        }
        result
    }
    pub fn get_para_real_value(&self, para_name: &ATTR) -> f64 {
        self.attribute.get(para_name).unwrap().real_value()
    }
    fn set_value(&mut self, value_dict: HashMap<ATTR, f64>) {
        for (name, value) in value_dict.iter() {
            self.attribute.get_mut(name).unwrap().set_value(*value);
        }
    }
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
    obj_info_dict: HashMap<ObjType, HashMap<i32, String>>,
    data: HashMap<(DATA, i32), Array1<f64>>,
}
impl DataStructOfDoExperiment {
    fn new(n: usize,
           obj_id_map: HashMap<String, (ObjType, i32)>,
           obj_info_dict: HashMap<ObjType, HashMap<i32, String>>,
        ) -> Self {
        DataStructOfDoExperiment {
            n,
            obj_id_map,
            obj_info_dict,
            data: HashMap::new(),
        }
    }
    pub fn add_data(&mut self, name: &str, data_name: &DATA, data: &Array1<f64>) {
        assert_eq!(data.len(), self.n);
        let obj_id = self.obj_id_map.get(name).unwrap().1;
        self.data.insert((data_name.clone(), obj_id), data.clone());
    }
    fn get_data(&self) -> &HashMap<(DATA, i32), Array1<f64>> {
        &self.data
    }
}
#[pyclass]
pub struct DataStructOfExpData {
    pub n: usize,
    pub repeat_time: usize,
    data: HashMap<(DATA, i32), ExpData>,
}
impl DataStructOfExpData {
    fn new(n: usize, repeat_time: usize, data: HashMap<(DATA, i32), ExpData>) -> Self {
        DataStructOfExpData {
            n,
            repeat_time,
            data
        }
    }
    pub fn get_data(&self) -> &HashMap<(DATA, i32), ExpData> {
        &self.data
    }
    pub fn get_t(&self) -> &ExpData {
        self.data.get(&(DATA::time(), 0)).unwrap()
    }
    pub fn get_data_by_name_id(&self, name: &str, id: i32) -> Result<&ExpData, String> {
        for (key, value) in self.data.iter() {
            if key.0.name() == name && key.1 == id {
                return Ok(value);
            }
        }
        Err(format!("Data {} not found", name))
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
    name: String,
    spdim: usize,
    exp_para: HashMap<String, Parastructure>,
    obj_info: HashMap<String, Objstructure>,
    data_info: HashMap<String, Vec<DATA>>,
}
impl ExpConfig {
    pub fn new(name: String, spdim: usize,
           exp_para: HashMap<String, Parastructure>,
           obj_info: HashMap<String, Objstructure>,
           data_info: HashMap<String, Vec<DATA>>) -> Self {
        ExpConfig {
            name,
            spdim,
            exp_para,
            obj_info,
            data_info,
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
    fn random_sample(&mut self) {
        for (_, para) in self.exp_para.iter_mut() {
            para.random_sample();
        }
        for (_, obj) in self.obj_info.iter_mut() {
            obj.random_sample();
        }
    }
    pub fn create_data_struct_of_do_experiment(&self, t_num: usize) -> DataStructOfDoExperiment {
        let mut obj_id_map: HashMap<String, (ObjType, i32)> = HashMap::new();
        let mut obj_info_dict: HashMap<ObjType, HashMap<i32, String>> = HashMap::new();
        for (name, obj) in self.obj_info.iter() {
            if obj.obj_type == ObjType::Clock {
                obj_id_map.insert(name.clone(), (ObjType::Clock, 0));
            }
        }
        let mut hash_vec: Vec<(&String, &Objstructure)> = self.obj_info.iter().collect();
        hash_vec.sort_by(|a, b| (a.0).cmp(b.0));
        for (name, obj) in hash_vec.iter() {
            let obj_type = obj.obj_type.clone();
            if obj_type == ObjType::Clock {
                continue;
            }
            let obj_id = obj_id_map.len() as i32;
            obj_id_map.insert((*name).clone(), (obj_type.clone(), obj_id));
            if !obj_info_dict.contains_key(&obj.obj_type) {
                obj_info_dict.insert(obj_type, HashMap::new());
            }
            obj_info_dict.get_mut(&obj.obj_type).unwrap().insert(obj_id, (*name).clone());
        }
        for (obj_name, data_names) in self.data_info.iter() {
            let obj_type = obj_id_map.get(obj_name).unwrap().clone().0;
            for data in data_names {
                assert_eq!(data.obj(), &obj_type);
            }
        }
        DataStructOfDoExperiment::new(
            t_num,
            obj_id_map,
            obj_info_dict,
        )
    }
}

#[pyclass]
pub struct ExpStructure {
    exp_config: ExpConfig,
    do_experiment: DoExpType,
}
impl ExpStructure {
    pub fn new(exp_config: ExpConfig, do_experiment: DoExpType) -> Self {
        ExpStructure {
            exp_config,
            do_experiment,
        }
    }
    pub fn print_obj_info(&self) {
        self.exp_config.print_obj_info();
    }
    pub fn random_sample(&mut self) {
        self.exp_config.random_sample();
    }
    pub fn get_expdata(&self, t_end: f64, t_num: usize, error: f64, repeat_time: usize) -> DataStructOfExpData {
        let doexp = self.do_experiment;
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
        DataStructOfExpData::new(t_num, repeat_time, multi_data)
    }
}

pub fn add_errors<D: Dimension>(array: &ArrayBase<OwnedRepr<f64>, D>, error: f64)
        -> ArrayBase<OwnedRepr<f64>, D> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, error).unwrap();
    array.mapv(|x| normal.sample(&mut rng) + x)
}
