use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{fmt, collections::HashMap};
use ndarray::{ArrayBase, Array1, Array2, Dimension, OwnedRepr};
use crate::experiments::expdata::ExpData;

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
    obj_type: String,
    attribute: HashMap<String, Parastructure>,
    data: HashMap<String, Parastructure>,
}
impl Objstructure {
    pub fn new(obj_type: String,
           attribute: HashMap<String, Parastructure>,
           data: HashMap<String, Parastructure>) -> Self {
        Objstructure {
            obj_type,
            attribute,
            data,
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
    fn measure(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        for (name, _) in self.attribute.iter() {
            result.insert(name.clone(), self.measure_para(name));
        }
        result
    }
    fn real_value(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        for (name, para) in self.attribute.iter() {
            result.insert(name.clone(), para.real_value());
        }
        result
    }
    fn get_para_real_value(&self, para_name: String) -> f64 {
        self.attribute.get(&para_name).unwrap().real_value()
    }
    fn set_value(&mut self, value_dict: HashMap<String, f64>) {
        for (name, value) in value_dict.iter() {
            self.attribute.get_mut(name).unwrap().set_value(*value);
        }
    }
}

impl fmt::Display for Objstructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Objstructure] obj_type: {}, attribute: {:?}, data: {:?}",
               self.obj_type, self.attribute.keys(), self.data.keys()).unwrap();
        write!(f, "\nattribute:").unwrap();
        for (name, para) in self.attribute.iter() {
            write!(f, "\n| {}: {}", name, para).unwrap();
        }
        Result::Ok(())
    }
}

pub trait ExpStructure {
    fn new() -> Self;
    fn name(&self) -> String;
    fn spdim(&self) -> usize;
    fn exp_para(&self) -> &HashMap<String, Parastructure>;
    fn mut_exp_para(&mut self) -> &mut HashMap<String, Parastructure>;
    fn obj_info(&self) -> &HashMap<String, Objstructure>;
    fn mut_obj_info(&mut self) -> &mut HashMap<String, Objstructure>;
    fn do_experiment(&self, t_end: f64, t_num: usize, error: f64) -> HashMap<String, Array1<f64>>;
    fn random_sample(&mut self) {
        for (_, para) in self.mut_exp_para().iter_mut() {
            para.random_sample();
        }
        for (_, obj) in self.mut_obj_info().iter_mut() {
            obj.random_sample();
        }
    }
    fn get_para_real_value(&self, para_name: String) -> f64 {
        self.exp_para().get(&para_name).unwrap().real_value()
    }
    fn get_obj_real_value(&self, obj_name: String, para_name: String) -> f64 {
        self.obj_info().get(&obj_name).unwrap().get_para_real_value(para_name)
    }
    fn get_expdata(&self, t_end: f64, t_num: usize, error: f64, repeat_time: usize) -> HashMap<String, ExpData> {
        let data: HashMap<String, Array1<f64>> = self.do_experiment(t_end, t_num, 0.0);
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