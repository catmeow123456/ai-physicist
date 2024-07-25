use std::collections::HashMap;
use crate::experiments::expstructure::{
    Parastructure, Objstructure
};
use crate::experiments::objects::obj::{
    ObjType::Spring, DATA, ATTR};

// 在一个实验中，每一个弹簧对象对应于两个可调节的旋钮，一个调节它的粗细（范围为 thickness_range ），另一个调节它的自由长度（范围为 freeL_range ）。

// make_spring，返回一个弹簧对象的结构，只可以调节它的粗细和自由长度。
impl Objstructure {
    pub fn spring(thickness_range: (f64, f64), freel_range: (f64, f64)) -> Self {
        assert!(thickness_range.0 < thickness_range.1 && freel_range.0 < freel_range.1);
        assert!(thickness_range.0 >= 1.5 && thickness_range.1 <= 2.5);
        assert!(freel_range.0 >= 1.0 && freel_range.1 <= 20.0);
        Objstructure::new(
            Spring,
            HashMap::from([
                (ATTR::thickness(),
                    Parastructure::new(Some(thickness_range))),
                (ATTR::freel(),
                    Parastructure::new(Some(freel_range))),
            ]),
        )
    }
}
impl ATTR {
    pub fn thickness() -> Self { ATTR::new(Spring, "thickness") }
    pub fn freel() -> Self { ATTR::new(Spring, "freel") }
}
impl DATA {
    pub fn posl() -> Self { DATA::new(Spring, "posl") }
    pub fn posr() -> Self { DATA::new(Spring, "posr") }
}
