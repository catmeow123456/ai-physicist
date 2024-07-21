use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};

// 在一个实验中，每一个质点对象对应于两个可调节的旋钮，一个调节它的质量（范围为 mass_range），另一个调节它的电荷（范围为 elec_range）

// make_masspoint，返回一个质点对象的结构，只可以调节它的质量，它没有电荷。
pub fn make_masspoint(mass_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        "MassPoint".to_string(),
        HashMap::from([
            ("mass".to_string(), Parastructure::new(Some(mass_range), None)),
            ("elec".to_string(), Parastructure::new(Some((0.0, 1e-8)), None)),
        ]),
        HashMap::from([
            ("posx".to_string(), Parastructure::new(None, None)),
            ("posy".to_string(), Parastructure::new(None, None)),
        ]),
    )
}