use crate::experiments::objects::spring::make_spring;
use crate::experiments::objects::masspoint::make_masspoint;
use crate::experiments::expstructure::{
    Parastructure, Objstructure, ExpStructure, add_normal_errors_to_array};
use ndarray::Array1;
use std::collections::HashMap;

pub struct Oscillation {
    exp_para: HashMap<String, Parastructure>,
    obj_info: HashMap<String, Objstructure>,
}
impl ExpStructure for Oscillation {
    fn new() -> Self {
        let default_masspoint_struct = make_masspoint((1.0, 5.0));
        let default_spring_struct = make_spring((2.0, 2.2), (9.0, 11.0));
        Oscillation {
            exp_para: HashMap::from([
                ("posl".to_string(), Parastructure::new(Some((-1.0, 1.0)), None)),
                ("x2".to_string(), Parastructure::new(Some((9.0, 11.0)), None)),
                ("v2".to_string(), Parastructure::new(Some((-2.0, 2.0)), None)),
            ]),
            obj_info: HashMap::from([
                ("MPa".to_string(), default_masspoint_struct),
                ("SPb".to_string(), default_spring_struct),
            ]),
        }
    }
    fn name(&self) -> String {"oscillation".to_string()}
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
        let x1 = self.get_para_real_value("posl".to_string());
        let x2 = self.get_para_real_value("x2".to_string());
        let v2 = self.get_para_real_value("v2".to_string());
        let mp1_mass_value = self.get_obj_real_value("MPa".to_string(), "mass".to_string());
        let sp2_length_value = self.get_obj_real_value("SPb".to_string(), "freeL".to_string());
        let sp2_k_value = (self.get_obj_real_value("SPb".to_string(), "thickness".to_string())).powf(3.0);
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let sp2_l = Array1::from_elem(t_num, x1);
        let omega = (sp2_k_value / mp1_mass_value).sqrt();
        let sp2_r = if x2 > x1 {
            &sp2_l + sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 - sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        } else {
            &sp2_l - sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 + sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        };
        HashMap::from([
            ("t".to_string(), add_normal_errors_to_array(&t, error)),
            ("MPa_pos".to_string(), add_normal_errors_to_array(&sp2_r, error)),
            ("SPb_posr".to_string(), add_normal_errors_to_array(&sp2_r, error)),
            ("SPb_posl".to_string(), add_normal_errors_to_array(&sp2_l, error)),
        ])
    }
}