use std::collections::HashMap;
use crate::experiments::expstructure::Objstructure;
use crate::experiments::objects::ObjType;

pub fn make_clock() -> Objstructure {
    Objstructure::new(
        ObjType::Clock,
        HashMap::from([]),
    )
}

pub static DATA_TIME: &str = "t";
pub static DATA_CLOCK: [&str; 1] = [DATA_TIME];