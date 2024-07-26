use pyo3::prelude::*;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);
use crate::ast::{Exp, SExp, Expression, UnaryOp, BinaryOp};
// mod ast;

#[pymethods]
impl Exp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}
#[pymethods]
impl SExp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

#[pyfunction]
fn parse_str(input: &str) -> PyResult<String> {
    let res = expr::ExpressionParser::new().parse(input).unwrap();
    match *res {
        Expression::Exp {exp} => Ok(format!("{}", *exp)),
        Expression::SExp {sexp} => Ok(format!("{}", *sexp)),
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
fn eval_exp(exp: &Exp, data: &DataStructOfExpData) -> PyResult<ExpData> {
    Ok(eval(exp, data))
}
#[pyfunction]
fn eval_sexp(sexp: &SExp, data: &DataStructOfExpData) -> PyResult<ExpData> {
    match sexp {
        SExp::Mk {name, exp} => {
            assert_eq!(data.name, *name);
            Ok(eval(exp, data))
        }
    }
}



use crate::experiments::expdata::ExpData;
use crate::experiments::expstructure::DataStructOfExpData;
pub fn eval(exp0: &Exp, context: &DataStructOfExpData) -> ExpData {
    let n = context.n;
    let repeat_time = context.repeat_time;
    match exp0 {
        Exp::Number {num} => ExpData::from_elem(*num as f64, n, repeat_time),
        Exp::Variable { name } => {
            assert_eq!(name, &"t".to_string());
            context.get_data_by_name_id(name, 0).unwrap().clone()
        }
        Exp::VariableId { name, id } => {
            context.get_data_by_name_id(name, *id).unwrap().clone()
        }
        Exp::UnaryExp { op: UnaryOp::Neg, ref exp } => -eval(&*exp, context),
        Exp::UnaryExp { op: UnaryOp::Diff, ref exp } => eval(&*exp, context).diff_tau(),
        Exp::BinaryExp { op: BinaryOp::Add, ref left, ref right } => eval(&*left, context) + eval(&*right, context),
        Exp::BinaryExp { op: BinaryOp::Sub, ref left, ref right } => eval(&*left, context) - eval(&*right, context),
        Exp::BinaryExp { op: BinaryOp::Mul, ref left, ref right } => eval(&*left, context) * eval(&*right, context),
        Exp::BinaryExp { op: BinaryOp::Div, ref left, ref right } => eval(&*left, context) / eval(&*right, context),
        Exp::BinaryExp { op: BinaryOp::Pow, ref left, ref right } => eval(&*left, context).pow(&eval(&*right, context)),
        Exp::DiffExp { ref left, ref right, ord} =>
            eval(&*left, context).diff_n(&eval(&*right, context), *ord as usize),
    }
}

#[pymodule]
pub fn register_sentence(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(m.py(), "sentence")?;
    child_module.add_function(wrap_pyfunction!(parse_str, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_sexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(eval_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(eval_sexp, m)?)?;
    m.add_submodule(&child_module)?;
    Ok(())
}