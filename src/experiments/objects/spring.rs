use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};

// 在一个实验中，每一个弹簧对象对应于两个可调节的旋钮，一个调节它的粗细（范围为 thickness_range ），另一个调节它的自由长度（范围为 freeL_range ）。

// make_spring，返回一个弹簧对象的结构，只可以调节它的粗细和自由长度。
pub fn make_spring(thickness_range: (f64, f64), freel_range: (f64, f64)) -> Objstructure {
    Objstructure::new(
        "Spring".to_string(),
        HashMap::from([
            ("thickness".to_string(), Parastructure::new(Some(thickness_range), None)),
            ("freeL".to_string(), Parastructure::new(Some(freel_range), None)),
        ]),
        HashMap::from([
            ("posl".to_string(), Parastructure::new(None, None)),
            ("posr".to_string(), Parastructure::new(None, None)),
        ]),
    )
}
