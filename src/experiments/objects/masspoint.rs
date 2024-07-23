use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::experiments::objects::obj::{
    ObjType::MassPoint, ATTR, DATA
};

// 在一个实验中，每一个质点对象对应于两个可调节的旋钮，一个调节它的质量（范围为 mass_range），另一个调节它的电荷（范围为 elec_range）

// make_masspoint，返回一个质点对象的结构，只可以调节它的质量，它没有电荷。
pub fn make_masspoint(mass_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        MassPoint,
        HashMap::from([
            (ATTR_MASS.clone(),
                Parastructure::new(Some(mass_range), None)),
            (ATTR_ELEC.clone(),
                Parastructure::new(Some((0.0, 1e-8)), None)),
        ]),
    )
}

pub static ATTR_MASS : ATTR = ATTR::new(MassPoint, "m");
pub static ATTR_ELEC : ATTR = ATTR::new(MassPoint, "e");
pub static DATA_POSX : DATA = DATA::new(MassPoint, "posx");
pub static DATA_POSY : DATA = DATA::new(MassPoint, "posy");
