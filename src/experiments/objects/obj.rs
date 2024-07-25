use pyo3::prelude::*;
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ObjType {
    Clock,
    MassPoint,
    Spring,
}
use std::fmt;
impl fmt::Display for ObjType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjType::Clock => write!(f, "Clock"),
            ObjType::MassPoint => write!(f, "MassPoint"),
            ObjType::Spring => write!(f, "Spring"),
        }
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DATA {
    Mk {
        obj: ObjType,
        name: String,
    },
}
impl DATA {
    pub fn new(obj: ObjType, name: &str) -> DATA {
        DATA::Mk { obj, name: name.to_string() }
    }
    pub fn name(&self) -> &String {
        match self {
            DATA::Mk { name, .. } => name,
        }
    }
    pub fn obj(&self) -> &ObjType {
        match self {
            DATA::Mk { obj, .. } => obj,
        }
    }
}
impl fmt::Display for DATA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DATA::Mk { obj: _, name } => write!(f, "{}", name),
        }
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ATTR {
    Mk {
        obj: ObjType,
        name: String,
    },
}
impl ATTR {
    pub fn new(obj: ObjType, name: &str) -> ATTR {
        ATTR::Mk { obj, name: name.to_string() }
    }
    pub fn name(&self) -> &String {
        match self {
            ATTR::Mk { name, .. } => name,
        }
    }
    pub fn obj(&self) -> &ObjType {
        match self {
            ATTR::Mk { obj, .. } => obj,
        }
    }
}
impl fmt::Display for ATTR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ATTR::Mk { name, .. } => write!(f, "{}", name),
        }
    }
}