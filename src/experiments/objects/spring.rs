use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::experiments::objects::ObjType;

// 在一个实验中，每一个弹簧对象对应于两个可调节的旋钮，一个调节它的粗细（范围为 thickness_range ），另一个调节它的自由长度（范围为 freeL_range ）。

// make_spring，返回一个弹簧对象的结构，只可以调节它的粗细和自由长度。
pub fn make_spring(thickness_range: (f64, f64), freel_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        ObjType::Spring,
        HashMap::from([
            (ATTR_THICKNESS, Parastructure::new(Some(thickness_range), None)),
            (ATTR_FREEL, Parastructure::new(Some(freel_range), None)),
        ])
    )
}
pub static ATTR_THICKNESS: &str = "thickness";
pub static ATTR_FREEL: &str = "freeL";
pub static ATTR_SPRING: [&str; 2] = [ATTR_THICKNESS, ATTR_FREEL];
pub static DATA_POSL: &str = "posl";
pub static DATA_POSR: &str = "posr";
pub static DATA_SPRING: [&str; 2] = [DATA_POSL, DATA_POSR];
