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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DATA {
    Mk {
        obj: ObjType,
        name: &'static str,
    },
}
impl DATA {
    pub const fn new(obj: ObjType, name: &'static str) -> DATA {
        DATA::Mk { obj, name }
    }
    pub fn name(&self) -> &'static str {
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


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ATTR {
    Mk {
        obj: ObjType,
        name: &'static str,
    },
}
impl ATTR {
    pub const fn new(obj: ObjType, name: &'static str) -> ATTR {
        ATTR::Mk { obj, name }
    }
    pub fn name(&self) -> &'static str {
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