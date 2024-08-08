use pyo3::prelude::*;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);
use crate::ast::{Exp, SExp, TExp, Expression};
// mod ast;

#[pymethods]
impl Expression {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

// Function for parsing a string into an expression
#[pyfunction]
fn parse(input: &str) -> PyResult<Expression> {
    let exp = expr::ExpressionParser::new().parse(input).unwrap();
    Ok(*exp)
}
#[pyfunction]
fn parse_str(input: &str) -> PyResult<String> {
    let res = expr::ExpressionParser::new().parse(input).unwrap();
    match *res {
        Expression::Exp {exp} => Ok(format!("{}", *exp)),
        Expression::SExp {sexp} => Ok(format!("{}", *sexp)),
        Expression::TExp {texp} => Ok(format!("{}", *texp)),
        Expression::ObjAttrExp {objattrexp} => Ok(format!("{}", *objattrexp)),
    }
}
#[pyfunction]
pub fn parse_exp(input: &str) -> PyResult<Exp> {
    let res: Box<Exp> = expr::ExpParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_sexp(input: &str) -> PyResult<SExp> {
    let res: Box<SExp> = expr::SExpParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_texp(input: &str) -> PyResult<TExp> {
    let res: Box<TExp> = expr::TExpParser::new().parse(input).unwrap();
    Ok(*res)
}


#[pymodule]
pub fn register_sentence(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(m.py(), "sentence")?;
    child_module.add_function(wrap_pyfunction!(parse, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_str, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_sexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_texp, m)?)?;
    m.add_submodule(&child_module)?;
    Ok(())
}