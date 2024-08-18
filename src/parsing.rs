use crate::r;
use pyo3::prelude::*;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);
use crate::ast::{Proposition, Exp, SExp, TExp, ObjAttrExp, Expression};
// mod ast;

#[pymethods]
impl Expression {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[getter]
    fn expr_type(&self) -> String {
        match self {
            Expression::Exp {exp: _} => r!("Exp"),
            Expression::SExp {sexp: _} => r!("SExp"),
            Expression::TExp {texp: _} => r!("TExp"),
            Expression::ObjAttrExp {objattrexp: _} => r!("ObjAttrExp"),
            Expression::Proposition {prop: _} => r!("Proposition"),
        }
    }
    fn unwrap_exp(&self) -> Exp {
        match self {
            Expression::Exp {exp} => *exp.clone(),
            _ => panic!("Not an Exp"),
        }
    }
    fn unwrap_texp(&self) -> TExp {
        match self {
            Expression::TExp {texp} => *texp.clone(),
            _ => panic!("Not a TExp"),
        }
    }
    fn unwrap_sexp(&self) -> SExp {
        match self {
            Expression::SExp {sexp} => *sexp.clone(),
            _ => panic!("Not a SExp"),
        }
    }
    fn unwrap_objattrexp(&self) -> ObjAttrExp {
        match self {
            Expression::ObjAttrExp {objattrexp} => *objattrexp.clone(),
            _ => panic!("Not an ObjAttrExp"),
        }
    }
    fn unwrap_proposition(&self) -> Proposition {
        match self {
            Expression::Proposition {prop} => *prop.clone(),
            _ => panic!("Not a Proposition"),
        }
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
        Expression::Proposition {prop} => Ok(format!("{}", *prop)),
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
#[pyfunction]
pub fn parse_objattrexp(input: &str) -> PyResult<ObjAttrExp> {
    let res: Box<ObjAttrExp> = expr::ObjAttrExpParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_proposition(input: &str) -> PyResult<Proposition> {
    // println!("Parsing proposition: {}", input);
    let res: Box<Proposition> = expr::PropositionParser::new().parse(input).unwrap();
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
    child_module.add_function(wrap_pyfunction!(parse_objattrexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_proposition, m)?)?;
    m.add_submodule(&child_module)?;
    Ok(())
}