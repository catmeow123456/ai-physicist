use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::experiments::objects::ObjType;

// 在一个实验中，每一个质点对象对应于两个可调节的旋钮，一个调节它的质量（范围为 mass_range），另一个调节它的电荷（范围为 elec_range）

// make_masspoint，返回一个质点对象的结构，只可以调节它的质量，它没有电荷。
pub fn make_masspoint(mass_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        ObjType::MassPoint,
        HashMap::from([
            (ATTR_MASS, Parastructure::new(Some(mass_range), None)),
            (ATTR_ELEC, Parastructure::new(Some((0.0, 1e-8)), None)),
        ]),
    )
}

pub static ATTR_MASS : &str = "mass";
pub static ATTR_ELEC : &str = "elec";
pub static ATTRS_MASSPOINT: [&str; 2] = [ATTR_MASS, ATTR_ELEC];
pub static DATA_POSX: &str = "posx";
pub static DATA_POSY: &str = "posy";
pub static DATA_MASSPOINT: [&str; 2] = [DATA_POSX, DATA_POSY];
