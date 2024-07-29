use pyo3::prelude::*;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);
use crate::ast::{Exp, SExp, IExpConfig, Expression, UnaryOp, BinaryOp, MeasureType};
use crate::experiments::expstructure::{DataStructOfExpData, ExpStructure, Objstructure};
use crate::experiments::expdata::ExpData;
use pyo3::exceptions::PyTypeError;
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

// Function for evaluating an expression
#[pyfunction]
fn eval_exp(exp: &Exp, data: &DataStructOfExpData) -> PyResult<ExpData> {
    Ok(eval(exp, data))
}
#[pyfunction]
fn eval_sexp(sexp: &SExp, data: &DataStructOfExpData) -> PyResult<ExpData> {
    match sexp {
        SExp::Mk {expconfig, exp} => {
            let ref expconfig = **expconfig;
            match expconfig {
                IExpConfig::From {name} => {
                    assert_eq!(data.name, *name);
                    Ok(eval(exp, data))
                }
                _ => Err(PyTypeError::new_err(format!(
                    "To evaluate sexp {}, objstructures must be provided, please use 'seval_sexp' instead",sexp)))
            }
        }
    }
}

#[pyfunction]
fn eval_iexpconfig(iexpconfig: &IExpConfig, expstructure: &mut ExpStructure, objsettings: Vec<Objstructure>,
                   t_end: f64, t_num: i32, error: f64, repeat_time: i32) -> PyResult<DataStructOfExpData> {
    // create_data_struct_of_do_experiment
    match iexpconfig {
        IExpConfig::Mk {objtype, expconfig, id} => {
            let mut objsettings = objsettings;
            let obj = objsettings.pop().unwrap();
            assert_eq!(*objtype, obj.obj_type.to_string());
            expstructure.set_obj(*id, obj);
            eval_iexpconfig(expconfig, expstructure, objsettings, t_end, t_num, error, repeat_time)
        }
        IExpConfig::From {name} => {
            assert_eq!(objsettings.len(), 0);
            assert_eq!(name, expstructure.name());
            Ok(expstructure.collect_expdata(MeasureType::new(t_end, t_num, repeat_time, error)))
        }
    }
}

#[pyfunction]
fn seval_sexp(sexp: &SExp, expstructure: &mut ExpStructure, objsettings: Vec<Objstructure>,
              t_end: f64, t_num: i32, error: f64, repeat_time: i32) -> PyResult<ExpData> {
    match sexp {
        SExp::Mk {expconfig, exp} => {
            let ref expconfig = **expconfig;
            let data = eval_iexpconfig(expconfig, expstructure, objsettings, t_end, t_num, error, repeat_time)?;
            eval_exp(exp, &data)
        }
    }
}

pub fn eval(exp0: &Exp, context: &DataStructOfExpData) -> ExpData {
    let n = context.measuretype.n();
    let repeat_time = context.measuretype.repeat_time();
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
        Exp::ExpWithMeasureType {exp:_, measuretype:_} => {
            unimplemented!()
        }
    }
}

#[pymodule]
pub fn register_sentence(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(m.py(), "sentence")?;
    child_module.add_function(wrap_pyfunction!(parse, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_str, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(parse_sexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(eval_exp, m)?)?;
    child_module.add_function(wrap_pyfunction!(eval_sexp, m)?)?;
    child_module.add_function(wrap_pyfunction!(eval_iexpconfig, m)?)?;
    child_module.add_function(wrap_pyfunction!(seval_sexp, m)?)?;
    m.add_submodule(&child_module)?;
    Ok(())
}