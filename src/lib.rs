// pub mod expstructure;
pub mod experiments{
    pub mod expdata;
    pub mod expstructure;
    pub mod objects{
        #[derive(Clone, PartialEq, Eq, Hash)]
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
        pub mod clock;
        pub mod masspoint;
        pub mod spring;
    }
    pub mod simulation{
        pub mod collision;
        pub mod oscillation;
    }
}