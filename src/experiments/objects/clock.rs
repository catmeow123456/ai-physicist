use crate::r;
use crate::ast::TExp;
use std::collections::HashMap;
use super::super::expstructure::Objstructure;
use super::obj::ObjType::Clock;
use super::obj::DATA;

impl Objstructure {
    pub fn clock() -> Self {
        Objstructure::new(
            Clock,
            HashMap::from([]),
        )
    }
}

impl DATA {
    pub fn time() -> TExp { DATA::data(vec![r!("Clock")], r!("t")) }
}
