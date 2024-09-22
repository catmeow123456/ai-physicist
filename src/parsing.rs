use crate::r;
use pyo3::prelude::*;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);

use crate::experiments::expstructure::Objstructure;
#[pyfunction]
pub fn parse_objstructure(input: &str) -> PyResult<Objstructure> {
    let res: Objstructure = expr::ObjstructureParser::new().parse(input).unwrap();
    Ok(res)
}
use crate::knowledge::Knowledge;
#[pyfunction]
pub fn parse_knowledge(input: &str) -> PyResult<Knowledge> {
    let res: Knowledge = expr::KnowledgeParser::new().parse(input).unwrap();
    Ok(res)
}


use crate::ast::{Proposition, AtomExp, Exp, SExp, Concept, IExpConfig, Intrinsic, Expression};
// mod ast;

#[pymethods]
impl Expression {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[new]
    fn from_string(input: &str) -> Self {
        let exp = expr::ExpressionParser::new().parse(input).unwrap();
        *exp
    }
    #[getter]#[inline]
    fn expr_type(&self) -> String {
        match self {
            Expression::Exp {exp: _} => r!("Exp"),
            Expression::SExp {sexp: _} => r!("SExp"),
            Expression::Concept {concept: _} => r!("Concept"),
            Expression::Intrinsic {intrinsic: _} => r!("Intrinsic"),
            Expression::Proposition {prop: _} => r!("Proposition"),
        }
    }
    #[getter]#[inline]
    fn unwrap_exp(&self) -> Exp {
        match self {
            Expression::Exp {exp} => *exp.clone(),
            _ => panic!("Not an Exp"),
        }
    }
    #[getter]#[inline]
    fn unwrap_concept(&self) -> Concept {
        match self {
            Expression::Concept {concept} => *concept.clone(),
            _ => panic!("Not a Concept"),
        }
    }
    #[getter]#[inline]
    fn unwrap_sexp(&self) -> SExp {
        match self {
            Expression::SExp {sexp} => *sexp.clone(),
            _ => panic!("Not a SExp"),
        }
    }
    #[getter]#[inline]
    fn unwrap_intrinsic(&self) -> Intrinsic {
        match self {
            Expression::Intrinsic {intrinsic} => *intrinsic.clone(),
            _ => panic!("Not an Intrinsic"),
        }
    }
    #[getter]#[inline]
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
        Expression::Concept {concept} => Ok(format!("{}", *concept)),
        Expression::Intrinsic {intrinsic} => Ok(format!("{}", *intrinsic)),
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
pub fn parse_concept(input: &str) -> PyResult<Concept> {
    let res: Box<Concept> = expr::ConceptParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_intrinsic(input: &str) -> PyResult<Intrinsic> {
    let res: Box<Intrinsic> = expr::IntrinsicParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_atomexp(input: &str) -> PyResult<AtomExp> {
    let res: Box<AtomExp> = expr::AtomExpParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_proposition(input: &str) -> PyResult<Proposition> {
    // println!("Parsing proposition: {}", input);
    let res: Box<Proposition> = expr::PropositionParser::new().parse(input).unwrap();
    Ok(*res)
}
#[pyfunction]
pub fn parse_iexpconfig(input: &str) -> PyResult<IExpConfig> {
    let res: Box<IExpConfig> = expr::ExpConfigParser::new().parse(input).unwrap();
    Ok(*res)
}

#[pymodule]
pub fn register_sentence(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(m.py(), "sentence")?;
    child_module.add_function(wrap_pyfunction!(parse, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_str, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_sexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_concept, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_intrinsic, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_proposition, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_iexpconfig, m)?)?;
    m.add_submodule(&child_module)?;
    Ok(())
}