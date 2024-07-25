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
pub fn struct_oscillation() -> ExpStructure {
    let default_masspoint_struct = Objstructure::masspoint((1.0, 5.0));
    let default_spring_struct = Objstructure::spring((2.0, 2.2), (9.0, 11.0));
    let name = r!("oscillation");
    let spdim = 1 as usize;
    let exp_para = HashMap::from([
        (r!("posl"), Parastructure::new(Some((-1.0, 1.0)))),
        (r!("x2"), Parastructure::new(Some((9.0, 11.0)))),
        (r!("v2"), Parastructure::new(Some((-2.0, 2.0)))),
    ]);
    let obj_info = HashMap::from([
        (r!("MPa"), default_masspoint_struct),
        (r!("SPb"), default_spring_struct),
        (r!("Clock"), Objstructure::clock()),
    ]);
    let data_info = HashMap::from([
        (r!("MPa"), vec![DATA::posx()]),
        (r!("SPb"), vec![DATA::posl(), DATA::posr()]),
        (r!("Clock"), vec![DATA::time()]),
    ]);
    let exp_config = ExpConfig::new(name, spdim, exp_para, obj_info, data_info);
    let do_experiment: DoExpType = |t_end: f64, t_num: usize, error: f64, exp_config: &ExpConfig| {
        let x1 = exp_config.para("posl");
        let x2 = exp_config.para("x2");
        let v2 = exp_config.para("v2");
        let mp1_mass_value = exp_config.obj_para("MPa", &ATTR::mass());
        let sp2_length_value = exp_config.obj_para("SPb", &ATTR::freel());
        let sp2_k_value = exp_config.obj_para("SPb", &ATTR::thickness()).powf(3.);
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let sp2_l = Array1::from_elem(t_num, x1);
        let omega = (sp2_k_value / mp1_mass_value).sqrt();
        let sp2_r = if x2 > x1 {
            &sp2_l + sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 - sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        } else {
            &sp2_l - sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 + sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        };
        let mut data_struct = exp_config.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data("Clock", &DATA::time(), &add_errors(&t, error));
        data_struct.add_data("MPa", &DATA::posx(), &add_errors(&sp2_r, error));
        data_struct.add_data("SPb", &DATA::posr(), &add_errors(&sp2_r, error));
        data_struct.add_data("SPb", &DATA::posl(), &add_errors(&sp2_l, error));
        data_struct
    };
    ExpStructure::new(exp_config, do_experiment)
}