pub mod ast;
pub mod sentence;
pub mod experiments{
    pub mod expdata;
    pub mod expstructure;
    pub mod objects{
        pub mod obj;
        pub mod clock;
        pub mod masspoint;
        pub mod spring;
    }
    pub mod simulation{
        pub mod collision;
        pub mod oscillation;
    }
}