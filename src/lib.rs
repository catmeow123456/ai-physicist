pub mod macros;
pub mod ast;
pub mod sentence;
pub mod knowledge;
pub mod test{
    pub mod test_experiments;
}
pub mod experiments{
    pub mod impl_for_pyo3;
    pub mod topy;
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

use experiments::{
    topy::register_experiment,
    expdata::ExpData,
    expstructure::DataStructOfExpData,
};
use pyo3::prelude::*;
use sentence::register_sentence;
use knowledge::Knowledge;

#[pymodule]
fn ai_physicist(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_sentence(m)?;
    m.add_class::<ast::Func>()?;
    m.add_class::<ast::BinaryOp>()?;
    m.add_class::<ast::UnaryOp>()?;
    m.add_class::<ast::Exp>()?;
    m.add_class::<ast::SExp>()?;
    m.add_class::<ast::IExpConfig>()?;
    m.add_class::<ast::ObjAttrExp>()?;
    m.add_class::<ast::MeasureType>()?;
    m.add_class::<ExpData>()?;
    m.add_class::<DataStructOfExpData>()?;
    m.add_class::<Knowledge>()?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::struct_collision, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::oscillation::struct_oscillation, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::do_collision, m)?)?;
    register_experiment(m)?;
    Ok(())
}
