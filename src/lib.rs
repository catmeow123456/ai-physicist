pub mod ast;
pub mod sentence;
pub mod test{
    pub mod test_experiments;
}
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

use pyo3::prelude::*;
use sentence::parse;
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

#[pymodule]
fn ai_physicist(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    m.add_class::<ast::Func>()?;
    m.add_class::<ast::BinaryOp>()?;
    m.add_class::<ast::UnaryOp>()?;
    m.add_class::<ast::Exp>()?;
    Ok(())
}
