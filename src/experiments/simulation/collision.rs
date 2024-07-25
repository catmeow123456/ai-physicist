use crate::experiments::{
    objects::obj::{DATA, ATTR},
    expstructure::{
        Parastructure, Objstructure, ExpStructure, ExpConfig, add_errors,
        DataStructOfDoExperiment
    }
};
use ndarray::Array1;
use std::collections::HashMap;

fn do_collision(m1: f64, m2: f64, v1: f64, v2: f64) -> (f64, f64) {
    let vn1 = (m1 - m2) / (m1 + m2) * v1 + 2.0 * m2 / (m1 + m2) * v2;
    let vn2 = 2.0 * m1 / (m1 + m2) * v1 + (m2 - m1) / (m1 + m2) * v2;
    (vn1, vn2)
}

pub fn struct_collision() -> ExpStructure {
    let default_mass_struct = Objstructure::masspoint((2.0, 10.0));
    let exp_para = HashMap::from([
        ("x1", Parastructure::new(Some((-4.0, -3.0)), None)),
        ("v1", Parastructure::new(Some((3.0, 5.0)), None)),
        ("x2", Parastructure::new(Some((3.0, 4.0)), None)),
        ("v2", Parastructure::new(Some((-5.0, -3.0)), None)),
    ]);
    let obj_info = HashMap::from([
        ("MPa", default_mass_struct.clone()),
        ("MPb", default_mass_struct),
        ("Clock", Objstructure::clock()),
    ]);
    let data_info = HashMap::from([
        ("MPa", vec![DATA::posx()]),
        ("MPb", vec![DATA::posx()]),
        ("Clock", vec![DATA::time()]),
    ]);
    let name = "collision".to_string();
    let spdim = 1 as usize;
    let exp_config = ExpConfig::new(name, spdim, exp_para, obj_info, data_info);
    let do_experiment: fn(f64,usize,f64,&ExpConfig) -> DataStructOfDoExperiment = |t_end: f64, t_num: usize, error: f64, exp_config: &ExpConfig| -> DataStructOfDoExperiment {
        let x1 = exp_config.para("x1");
        let x2 = exp_config.para("x2");
        let v1 = exp_config.para("v1");
        let v2 = exp_config.para("v2");
        let m1 = exp_config.obj_para("MPa", &ATTR::mass());
        let m2 = exp_config.obj_para("MPb", &ATTR::mass());
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let t_collision = (x2 - x1) / (v1 - v2);
        assert!(t_collision > 0.0);
        assert!(t_collision < t_end);
        let (vn1, vn2) = do_collision(m1, m2, v1, v2);
        let data_x1: Array1<f64> = t.mapv(|t| if t < t_collision {x1 + v1 * t} else {x1 + v1 * t_collision + vn1 * (t - t_collision)});
        let data_x2: Array1<f64> = t.mapv(|t| if t < t_collision {x2 + v2 * t} else {x2 + v2 * t_collision + vn2 * (t - t_collision)});
        let mut data_struct = exp_config.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data("Clock", &DATA::time(), &add_errors(&t, error));
        data_struct.add_data("MPa", &DATA::posx(), &add_errors(&data_x1, error));
        data_struct.add_data("MPb", &DATA::posx(), &add_errors(&data_x2, error));
        data_struct
    };
    ExpStructure::new(exp_config, do_experiment)
}