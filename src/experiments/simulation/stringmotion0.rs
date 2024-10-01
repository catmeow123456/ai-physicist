use pyo3::prelude::*;
use crate::r;
use crate::experiments::{
    objects::obj::{DATA, ATTR},
    expstructure::{
        Parastructure, Objstructure, ExpStructure, ExpConfig, add_errors, DoExpType
    }
};
use ndarray::Array1;
use std::collections::HashMap;

#[pyfunction]
pub fn struct_stringmotion0() -> ExpStructure {
    let default_spring_struct = Objstructure::spring((1.5, 2.5), (1.0, 20.0));
    let name = r!("stringmotion0");
    let spdim = 1 as usize;
    let exp_para = HashMap::from([
        (r!("posl"), Parastructure::new(Some((-1.0, 1.0)))),
        (r!("v0"), Parastructure::new(Some((-2.0, 2.0)))),
    ]);
    let obj_info = HashMap::from([
        (r!("SPa"), default_spring_struct),
        (r!("Clock"), Objstructure::clock()),
    ]);
    let data_info = vec![
        (DATA::posl(), vec![r!("SPa")]),
        (DATA::posr(), vec![r!("SPa")]),
        (DATA::time(), vec![r!("Clock")]),
    ];
    let exp_config = ExpConfig::new(name, spdim, exp_para, obj_info, data_info);
    let do_experiment: DoExpType = |t_end: f64, t_num: usize, error: f64, exp_config: &ExpConfig| {
        let x0 = exp_config.para("posl");
        let v0 = exp_config.para("v0");
        let spa_length_value = exp_config.obj_para("SPa", &ATTR::freel());
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let spl = x0 + v0 * &t;
        let spr = x0 + spa_length_value + v0 * &t;
        let mut data_struct = exp_config.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data((DATA::time(), vec![r!("Clock")]), &add_errors(&t, error));
        data_struct.add_data((DATA::posr(), vec![r!("SPa")]), &add_errors(&spr, error));
        data_struct.add_data((DATA::posl(), vec![r!("SPa")]), &add_errors(&spl, error));
        data_struct
    };
    ExpStructure::new(exp_config, do_experiment)
}