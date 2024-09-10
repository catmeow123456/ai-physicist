pub mod macros;
pub mod ast;
pub mod ast_display;
pub mod exprcharacter;
pub mod parsing;
pub mod knowledge;
pub mod regression;
pub mod complexity;
pub mod expdata{
    pub mod topy;
    pub mod expdata;
    pub mod normaldata;
    pub mod constdata;
}
pub mod experiments{
    pub mod topy;
    pub mod expstructure;
    pub mod objects{
        pub mod obj;
        pub mod clock;
        pub mod masspoint;
        pub mod spring;
    }
    pub mod simulation{
        pub mod motion0;
        pub mod motion;
        pub mod collision;
        pub mod oscillation;
    }
}

pub mod impl_for_pyo3;
use expdata::topy::register_data;
use experiments::topy::register_experiment;
use pyo3::prelude::*;
use parsing::register_sentence;
use knowledge::Knowledge;
use regression::{search_relations, search_relations_ver2, search_trivial_relations};

#[pymodule]
fn ai_physicist(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_sentence(m)?;
    m.add_class::<ast::Func>()?;
    m.add_class::<ast::BinaryOp>()?;
    m.add_class::<ast::UnaryOp>()?;
    m.add_class::<ast::Proposition>()?;
    m.add_class::<ast::Exp>()?;
    m.add_class::<ast::SExp>()?;
    m.add_class::<ast::Concept>()?;
    m.add_class::<ast::AtomExp>()?;
    m.add_class::<ast::Expression>()?;
    m.add_class::<ast::IExpConfig>()?;
    m.add_class::<ast::Intrinsic>()?;
    m.add_class::<ast::MeasureType>()?;
    m.add_class::<Knowledge>()?;
    m.add_function(wrap_pyfunction!(search_relations, m)?)?;
    m.add_function(wrap_pyfunction!(search_relations_ver2, m)?)?;
    m.add_function(wrap_pyfunction!(search_trivial_relations, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::motion0::struct_motion0, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::motion::struct_motion, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::struct_collision, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::oscillation::struct_oscillation, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::do_collision, m)?)?;
    register_experiment(m)?;
    register_data(m)?;
    Ok(())
}
