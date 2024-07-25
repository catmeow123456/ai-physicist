pub mod ast;
pub mod sentence;
pub mod macros;
pub mod test{
    pub mod test_experiments;
}
pub mod experiments{
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
    topy::register_experiment_maker,
    expdata::ExpData,
    expstructure::DataStructOfExpData,
};
use pyo3::prelude::*;
use sentence::{parse, eval};
#[pyfunction]
fn parse_str(input: &str) -> PyResult<String> {
    let res = parse(input).unwrap();
    Ok(format!("{}", res))
}
#[pyfunction]
fn parse_exp(input: &str) -> PyResult<ast::Exp> {
    let res = parse(input).unwrap();
    Ok(res)
}
#[pyfunction]
fn eval_exp(exp: &ast::Exp, data: &DataStructOfExpData) -> PyResult<ExpData> {
    Ok(eval(exp, data))
}


#[pymodule]
fn ai_physicist(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    m.add_class::<ast::Func>()?;
    m.add_class::<ast::BinaryOp>()?;
    m.add_class::<ast::UnaryOp>()?;
    m.add_class::<ast::Exp>()?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::struct_collision, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::oscillation::struct_oscillation, m)?)?;
    m.add_class::<ExpData>()?;
    m.add_class::<DataStructOfExpData>()?;
    m.add_function(wrap_pyfunction!(eval_exp, m)?)?;
    m.add_function(wrap_pyfunction!(experiments::simulation::collision::do_collision, m)?)?;
    register_experiment_maker(m)?;
    Ok(())
}
