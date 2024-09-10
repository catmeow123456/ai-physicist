use crate::r;
use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::ast::Concept;
use crate::experiments::objects::obj::{
    ObjType::MassPoint, ATTR, DATA
};

// 在一个实验中，每一个质点对象对应于两个可调节的旋钮，一个调节它的质量（范围为 mass_range），另一个调节它的电荷（范围为 elec_range）

// make_masspoint，返回一个质点对象的结构，只可以调节它的质量，它没有电荷。
impl Objstructure {
    pub fn masspoint(mass_range: (f64, f64)) -> Self {
        assert!(mass_range.0 >= 1.0 && mass_range.1 <= 1000.0);
        Objstructure::new(
            MassPoint,
            HashMap::from([
                (ATTR::mass(),
                    Parastructure::new(Some(mass_range))),
                (ATTR::elec(),
                    Parastructure::new(Some((0.0, 1e-8)))),
            ]),
        )
    }

}
impl ATTR {
    pub fn mass() -> Self { ATTR::new(MassPoint,"m") }
    pub fn elec() -> Self { ATTR::new(MassPoint,"e") }
}
impl DATA {
    pub fn posx() -> Concept { DATA::data(vec![r!("MassPoint")], r!("posx")) }
    pub fn posy() -> Concept { DATA::data(vec![r!("MassPoint")], r!("posy")) }
}
