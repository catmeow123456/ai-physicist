use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{fmt, collections::HashMap};
use ndarray::{ArrayBase, Array1, Array2, Dimension, OwnedRepr};
use crate::experiments::expdata::ExpData;
use crate::experiments::objects::ObjType;

// 刻画某个参数结构的抽象类
// range: 参数的取值范围
// error: 测量它会带来的误差
#[derive(Clone)]
pub struct Parastructure {
    value: Option<f64>,
    pub range: (f64, f64),
    error: f64,
}

impl Parastructure {
    pub fn new(range: Option<(f64, f64)>, error: Option<f64>) -> Self {
        Parastructure {
            value: None,
            range: match range {
                Some(range) => range,
                None => (-1e10, 1e10),
            },
            error: match error {
                Some(error) => error,
                None => 1e-8,
            }
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
    fn measure(&self) -> f64 {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(self.value.unwrap(), self.error).unwrap();
        let value = normal.sample(&mut rng);
        value
    }
    fn real_value(&self) -> f64 {
        self.value.unwrap()
    }
}

impl fmt::Display for Parastructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Parastructure] value: {:?}, range: {:?}, error: {}",
               self.value, self.range, self.error).unwrap();
        Result::Ok(())
    }
}

#[derive(Clone)]
pub struct Objstructure {
    obj_type: ObjType,
    attribute: HashMap<&'static str, Parastructure>,
}
impl Objstructure {
    pub fn new(obj_type: ObjType,
           attribute: HashMap<&'static str, Parastructure>) -> Self {
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
    fn measure_para(&self, para_name: &str) -> f64 {
        self.attribute.get(para_name).unwrap().measure()
    }
    fn measure(&self) -> HashMap<&'static str, f64> {
        let mut result = HashMap::new();
        for (name, _) in self.attribute.iter() {
            result.insert(*name, self.measure_para(name));
        }
        result
    }
    fn real_value(&self) -> HashMap<&'static str, f64> {
        let mut result = HashMap::new();
        for (name, para) in self.attribute.iter() {
            result.insert(*name, para.real_value());
        }
        result
    }
    fn get_para_real_value(&self, para_name: &str) -> f64 {
        self.attribute.get(&para_name).unwrap().real_value()
    }
    fn set_value(&mut self, value_dict: HashMap<&'static str, f64>) {
        for (name, value) in value_dict.iter() {
            self.attribute.get_mut(name).unwrap().set_value(*value);
        }
    }
}

impl fmt::Display for Objstructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Objstructure] obj_type: {}, attribute: {:?}",
               self.obj_type, self.attribute.keys()).unwrap();
        write!(f, "\nattribute:").unwrap();
        for (name, para) in self.attribute.iter() {
            write!(f, "\n| {}: {}", name, para).unwrap();
        }
        Result::Ok(())
    }
}

pub struct DataStructOfDoExperiment {
    obj_id_map: HashMap<&'static str, (ObjType, usize)>,
    obj_info_dict: HashMap<ObjType, HashMap<usize, &'static str>>,
    data_info_dict: HashMap<(&'static str, &'static str), String>,
    data: HashMap<String, Array1<f64>>,
}
impl DataStructOfDoExperiment {
    fn new(obj_id_map: HashMap<&'static str, (ObjType, usize)>,
           obj_info_dict: HashMap<ObjType, HashMap<usize, &'static str>>,
           data_info_dict: HashMap<(&'static str, &'static str), String>) -> Self {
        DataStructOfDoExperiment {
            obj_id_map,
            obj_info_dict,
            data_info_dict,
            data: HashMap::new(),
        }
    }
    pub fn add_data(&mut self, name: &str, data_name: &str, data: &Array1<f64>) {
        let key = self.data_info_dict.get(&(name, data_name)).unwrap();
        self.data.insert(key.clone(), data.clone());
    }
    fn get_data(&self) -> &HashMap<String, Array1<f64>> {
        assert_eq!(self.data_info_dict.len(), self.data.len());
        &self.data
    }
}

pub trait ExpStructure {
    fn new() -> Self;
    fn name(&self) -> String;
    fn spdim(&self) -> usize;
    fn exp_para(&self) -> &HashMap<&'static str, Parastructure>;
    fn mut_exp_para(&mut self) -> &mut HashMap<&'static str, Parastructure>;
    fn obj_info(&self) -> &HashMap<&'static str, Objstructure>;
    fn mut_obj_info(&mut self) -> &mut HashMap<&'static str, Objstructure>;
    fn data_info(&self) -> &HashMap<&'static str, Vec<&'static str>>;
    fn create_data_struct_of_do_experiment(&self) -> DataStructOfDoExperiment {
        let mut obj_id_map: HashMap<&'static str, (ObjType, usize)> = HashMap::new();
        let mut obj_info_dict: HashMap<ObjType, HashMap<usize, &'static str>> = HashMap::new();
        let mut data_info_dict: HashMap<(&'static str, &'static str), String> = HashMap::new();
        for (name, obj) in self.obj_info().iter() {
            let obj_type = obj.obj_type.clone();
            let obj_id = if obj_type == ObjType::Clock {
                0
            } else {
                obj_id_map.len() + 1
            };
            obj_id_map.insert(name, (obj_type.clone(), obj_id));
            if !obj_info_dict.contains_key(&obj_type) {
                obj_info_dict.insert(obj_type.clone(), HashMap::new());
            }
            obj_info_dict.get_mut(&obj_type).unwrap().insert(obj_id, name);
        }
        for (obj_name, data_names) in self.data_info().iter() {
            let obj_id = obj_id_map.get(obj_name).unwrap().1;
            for data_name in data_names {
                let var_name = if obj_id == 0 {
                    format!("{}", data_name)
                } else {
                    format!("{}_{}", data_name, obj_id)
                };
                data_info_dict.insert((obj_name, data_name), var_name);
            }
        }
        DataStructOfDoExperiment::new(
            obj_id_map,
            obj_info_dict,
            data_info_dict,
        )
    }
    fn do_experiment(&self, t_end: f64, t_num: usize, error: f64) -> DataStructOfDoExperiment;
    fn random_sample(&mut self) {
        for (_, para) in self.mut_exp_para().iter_mut() {
            para.random_sample();
        }
        for (_, obj) in self.mut_obj_info().iter_mut() {
            obj.random_sample();
        }
    }
    fn get_para_real_value(&self, para_name: &str) -> f64 {
        self.exp_para().get(&para_name).unwrap().real_value()
    }
    fn get_obj_real_value(&self, obj_name: &str, para_name: &str) -> f64 {
        self.obj_info().get(&obj_name).unwrap().get_para_real_value(para_name)
    }
    fn get_expdata(&self, t_end: f64, t_num: usize, error: f64, repeat_time: usize) -> HashMap<String, ExpData> {
        let data_struct = self.do_experiment(t_end, t_num, 0.0);
        let data = data_struct.get_data();
        let mut multi_data: HashMap<String, ExpData> = HashMap::new();
        for (name, data) in data.iter() {
            let mut idata: Array2<f64> = Array2::zeros((repeat_time, t_num));
            for i in 0..repeat_time {
                for j in 0..t_num {
                    idata[[i, j]] = data[j]
                }
            }
            idata = add_normal_errors_to_array(&idata, error);
            assert_eq!(idata.shape(), [repeat_time, t_num]);
            multi_data.insert(name.clone(), ExpData::new(idata));
        }
        multi_data
    }
}

pub fn add_normal_errors_to_array<D: Dimension>(array: &ArrayBase<OwnedRepr<f64>, D>, error: f64)
        -> ArrayBase<OwnedRepr<f64>, D> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, error).unwrap();
    array.mapv(|x| normal.sample(&mut rng) + x)
}