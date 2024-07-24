use crate::experiments::objects::obj::{DATA, ATTR};
use crate::experiments::objects::spring::make_spring;
use crate::experiments::objects::masspoint::make_masspoint;
use crate::experiments::objects::clock::{make_clock, DATA_TIME};
use crate::experiments::expstructure::{
    Parastructure, Objstructure, ExpStructure, add_normal_errors_to_array,
    DataStructOfDoExperiment
};
use ndarray::Array1;
use std::collections::HashMap;

pub struct Oscillation {
    exp_para: HashMap<&'static str, Parastructure>,
    obj_info: HashMap<&'static str, Objstructure>,
    data_info: HashMap<&'static str, Vec<DATA>>,
}
impl ExpStructure for Oscillation {
    fn new() -> Self {
        let default_masspoint_struct = make_masspoint((1.0, 5.0));
        let default_spring_struct = make_spring((2.0, 2.2), (9.0, 11.0));
        Oscillation {
            exp_para: HashMap::from([
                ("posl", Parastructure::new(Some((-1.0, 1.0)), None)),
                ("x2", Parastructure::new(Some((9.0, 11.0)), None)),
                ("v2", Parastructure::new(Some((-2.0, 2.0)), None)),
            ]),
            obj_info: HashMap::from([
                ("MPa", default_masspoint_struct),
                ("SPb", default_spring_struct),
                ("Clock", make_clock()),
            ]),
            data_info: HashMap::from([
                ("MPa", vec![DATA::posx()]),
                ("SPb", vec![DATA::posl(), DATA::posr()]),
                ("Clock", vec![DATA_TIME.clone()]),
            ]),
        }
    }
    fn name(&self) -> String {"oscillation".to_string()}
    fn spdim(&self) -> usize {1}
    fn exp_para(&self) -> &HashMap<&'static str, Parastructure> {
        &self.exp_para
    }
    fn mut_exp_para(&mut self) -> &mut HashMap<&'static str, Parastructure> {
        &mut self.exp_para
    }
    fn obj_info(&self) -> &HashMap<&'static str, Objstructure> {
        &self.obj_info
    }
    fn mut_obj_info(&mut self) -> &mut HashMap<&'static str, Objstructure> {
        &mut self.obj_info
    }
    fn data_info(&self) -> &HashMap<&'static str, Vec<DATA>> {
        &self.data_info
    }
    fn do_experiment(&self, t_end: f64, t_num: usize, error: f64) -> DataStructOfDoExperiment {
        let x1 = self.get_para_real_value("posl");
        let x2 = self.get_para_real_value("x2");
        let v2 = self.get_para_real_value("v2");
        let mp1_mass_value = self.get_obj_real_value("MPa", &ATTR::mass());
        let sp2_length_value = self.get_obj_real_value("SPb", &ATTR::freel());
        let sp2_k_value = (self.get_obj_real_value("SPb", &ATTR::thickness())).powf(3.0);
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let sp2_l = Array1::from_elem(t_num, x1);
        let omega = (sp2_k_value / mp1_mass_value).sqrt();
        let sp2_r = if x2 > x1 {
            &sp2_l + sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 - sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        } else {
            &sp2_l - sp2_length_value + (v2 / omega) * (omega * &t).mapv(|x| x.sin()) + (x2 - x1 + sp2_length_value) * (omega * &t).mapv(|x| x.cos())
        };
        let mut data_struct = self.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data("Clock", &DATA_TIME, &add_normal_errors_to_array(&t, error));
        data_struct.add_data("MPa", &DATA::posx(), &add_normal_errors_to_array(&sp2_r, error));
        data_struct.add_data("SPb", &DATA::posr(), &add_normal_errors_to_array(&sp2_r, error));
        data_struct.add_data("SPb", &DATA::posl(), &add_normal_errors_to_array(&sp2_l, error));
        data_struct
    }
}