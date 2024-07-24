use crate::experiments::expstructure::{
    Parastructure, Objstructure, ExpStructure, add_errors,
    DataStructOfDoExperiment
};
use crate::experiments::objects::obj::{DATA, ATTR};
use ndarray::Array1;
use std::collections::HashMap;

pub struct Collision {
    exp_para: HashMap<&'static str, Parastructure>,
    obj_info: HashMap<&'static str, Objstructure>,
    data_info: HashMap<&'static str, Vec<DATA>>,
}

fn do_collision(m1: f64, m2: f64, v1: f64, v2: f64) -> (f64, f64) {
    let vn1 = (m1 - m2) / (m1 + m2) * v1 + 2.0 * m2 / (m1 + m2) * v2;
    let vn2 = 2.0 * m1 / (m1 + m2) * v1 + (m2 - m1) / (m1 + m2) * v2;
    (vn1, vn2)
}

impl ExpStructure for Collision {
    fn new() -> Self {
        let default_mass_struct = Objstructure::masspoint((2.0, 10.0));
        Collision {
            exp_para: HashMap::from([
                ("x1", Parastructure::new(Some((-4.0, -3.0)), None)),
                ("v1", Parastructure::new(Some((3.0, 5.0)), None)),
                ("x2", Parastructure::new(Some((3.0, 4.0)), None)),
                ("v2", Parastructure::new(Some((-5.0, -3.0)), None)),
            ]),
            obj_info: HashMap::from([
                ("MPa", default_mass_struct.clone()),
                ("MPb", default_mass_struct),
                ("Clock", Objstructure::clock()),
            ]),
            data_info: HashMap::from([
                ("MPa", vec![DATA::posx()]),
                ("MPb", vec![DATA::posx()]),
                ("Clock", vec![DATA::time()]),
            ]),
        }
    }
    fn name(&self) -> String {"collision".to_string()}
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
        let x1 = self.get_para_real_value("x1");
        let x2 = self.get_para_real_value("x2");
        let v1 = self.get_para_real_value("v1");
        let v2 = self.get_para_real_value("v2");
        let m1 = self.get_obj_real_value("MPa", &ATTR::mass());
        let m2 = self.get_obj_real_value("MPb", &ATTR::elec());
        let step = (t_end - 0.0) / (t_num as f64);
        let t: Array1<f64> = Array1::range(0.0, t_end, step);
        let t_collision = (x2 - x1) / (v1 - v2);
        assert!(t_collision > 0.0);
        assert!(t_collision < t_end);
        let (vn1, vn2) = do_collision(m1, m2, v1, v2);
        let data_x1: Array1<f64> = t.mapv(|t| if t < t_collision {x1 + v1 * t} else {x1 + v1 * t_collision + vn1 * (t - t_collision)});
        let data_x2: Array1<f64> = t.mapv(|t| if t < t_collision {x2 + v2 * t} else {x2 + v2 * t_collision + vn2 * (t - t_collision)});
        let mut data_struct = self.create_data_struct_of_do_experiment(t_num);
        data_struct.add_data("Clock", &DATA::time(), &add_errors(&t, error));
        data_struct.add_data("MPa", &DATA::posx(), &add_errors(&data_x1, error));
        data_struct.add_data("MPb", &DATA::posx(), &add_errors(&data_x2, error));
        data_struct
    }
}