use std::collections::HashMap;
use crate::experiments::expstructure::Objstructure;
use crate::experiments::objects::obj::ObjType;

use super::obj::DATA;

pub fn make_clock() -> Objstructure {
    Objstructure::new(
        ObjType::Clock,
        HashMap::from([]),
    )
}

pub static DATA_TIME: DATA = DATA::new(ObjType::Clock, "t");
