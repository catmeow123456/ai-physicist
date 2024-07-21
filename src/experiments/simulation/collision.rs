use crate::experiments::objects::masspoint::make_masspoint;
use crate::experiments::expstructure::{
    Parastructure, Objstructure, ExpStructure, add_normal_errors_to_array};
use ndarray::Array1;
use std::collections::HashMap;

pub struct Collision {
    exp_para: HashMap<String, Parastructure>,
    obj_info: HashMap<String, Objstructure>,
}

fn do_collision(m1: f64, m2: f64, v1: f64, v2: f64) -> (f64, f64) {
    let vn1 = (m1 - m2) / (m1 + m2) * v1 + 2.0 * m2 / (m1 + m2) * v2;
    let vn2 = 2.0 * m1 / (m1 + m2) * v1 + (m2 - m1) / (m1 + m2) * v2;
    (vn1, vn2)
}

impl ExpStructure for Collision {
    fn new() -> Self {
        let default_mass_struct = make_masspoint((2.0, 10.0));
        Collision {
            exp_para: HashMap::from([
                ("x1".to_string(), Parastructure::new(Some((-4.0, -3.0)), None)),
                ("v1".to_string(), Parastructure::new(Some((3.0, 5.0)), None)),
                ("x2".to_string(), Parastructure::new(Some((3.0, 4.0)), None)),
                ("v2".to_string(), Parastructure::new(Some((-5.0, -3.0)), None)),
            ]),
            obj_info: HashMap::from([
                ("MPa".to_string(), default_mass_struct.clone()),
                ("MPb".to_string(), default_mass_struct),
            ]),
        }
    }
    fn name(&self) -> String {"collision".to_string()}
    fn spdim(&self) -> usize {1}
    fn exp_para(&self) -> &HashMap<String, Parastructure> {
        &self.exp_para
    }
    fn mut_exp_para(&mut self) -> &mut HashMap<String, Parastructure> {
        &mut self.exp_para
    }
    fn obj_info(&self) -> &HashMap<String, Objstructure> {
        &self.obj_info
    }
    fn mut_obj_info(&mut self) -> &mut HashMap<String, Objstructure> {
        &mut self.obj_info
    }
    fn do_experiment(&self, t_end: f64, t_num: usize, error: f64) -> HashMap<String, Array1<f64>> {
        let x1 = self.get_para_real_value("x1".to_string());
        let x2 = self.get_para_real_value("x2".to_string());
        let v1 = self.get_para_real_value("v1".to_string());
        let v2 = self.get_para_real_value("v2".to_string());
        let m1 = self.get_obj_real_value("MPa".to_string(), "mass".to_string());
        let m2 = self.get_obj_real_value("MPb".to_string(), "mass".to_string());
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let t_collision = (x2 - x1) / (v1 - v2);
        assert!(t_collision > 0.0);
        assert!(t_collision < t_end);
        let (vn1, vn2) = do_collision(m1, m2, v1, v2);
        let data_x1: Array1<f64> = t.mapv(|t| if t < t_collision {x1 + v1 * t} else {x1 + v1 * t_collision + vn1 * (t - t_collision)});
        let data_x2: Array1<f64> = t.mapv(|t| if t < t_collision {x2 + v2 * t} else {x2 + v2 * t_collision + vn2 * (t - t_collision)});
        HashMap::from([
            ("t".to_string(), add_normal_errors_to_array(&t, error)),
            ("x1".to_string(), add_normal_errors_to_array(&data_x1, error)),
            ("x2".to_string(), add_normal_errors_to_array(&data_x2, error)),
        ])
    }
}