use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::experiments::objects::obj::{
    ObjType::Spring, DATA, ATTR};

// 在一个实验中，每一个弹簧对象对应于两个可调节的旋钮，一个调节它的粗细（范围为 thickness_range ），另一个调节它的自由长度（范围为 freeL_range ）。

// make_spring，返回一个弹簧对象的结构，只可以调节它的粗细和自由长度。
pub fn make_spring(thickness_range: (f64, f64), freel_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        Spring,
        HashMap::from([
            (ATTR_THICKNESS.clone(),
                Parastructure::new(Some(thickness_range), None)),
            (ATTR_FREEL.clone(),
                Parastructure::new(Some(freel_range), None)),
        ])
    )
}

pub static ATTR_THICKNESS: ATTR = ATTR::new(Spring, "thickness");
pub static ATTR_FREEL: ATTR = ATTR::new(Spring, "freel");
pub static DATA_POSL: DATA = DATA::new(Spring, "posl");
pub static DATA_POSR: DATA = DATA::new(Spring, "posr");
