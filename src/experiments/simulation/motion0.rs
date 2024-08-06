use pyo3::prelude::*;
use crate::r;
use crate::experiments::{
    objects::obj::DATA,
    expstructure::{
        Parastructure, Objstructure, ExpStructure, ExpConfig, add_errors, DoExpType
    }
};
use ndarray::Array1;
use std::collections::HashMap;

#[pyfunction]
pub fn struct_motion0() -> ExpStructure {
    let default_masspoint_struct = Objstructure::masspoint((1.0, 1000.0));
    let name = r!("motion0");
    let spdim = 1 as usize;
    let exp_para = HashMap::from([
        (r!("x0"), Parastructure::new(Some((9.0, 11.0)))),
        (r!("v0"), Parastructure::new(Some((-2.0, 2.0)))),
    ]);
    let obj_info = HashMap::from([
        (r!("MPa"), default_masspoint_struct),
        (r!("Clock"), Objstructure::clock()),
    ]);
    let data_info = HashMap::from([
        (r!("MPa"), vec![DATA::posx()]),
        (r!("Clock"), vec![DATA::time()]),
    ]);
    let exp_config = ExpConfig::new(name, spdim, exp_para, obj_info, data_info);
    let do_experiment: DoExpType = |t_end: f64, t_num: usize, error: f64, exp_config: &ExpConfig| {
        let x0 = exp_config.para("x0");
        let v0 = exp_config.para("v0");
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let x: Array1<f64> = x0 + v0 * &t;
        let mut data_struct = exp_config.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data("Clock", &DATA::time(), &add_errors(&t, error));
        data_struct.add_data("MPa", &DATA::posx(), &add_errors(&x, error));
        data_struct
    };
    ExpStructure::new(exp_config, do_experiment)
}