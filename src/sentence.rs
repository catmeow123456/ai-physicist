use lalrpop_util::lalrpop_mod;
lalrpop_mod!(expr);
use crate::ast;
// mod ast;
pub fn parse(input: &str) -> Result<ast::Exp, String> {
    let res: Box<ast::Exp> = expr::ExpParser::new().parse(input).unwrap();
    Ok(*res)
}

use crate::experiments::expdata::ExpData;
use crate::experiments::expstructure::DataStructOfExpData;
pub fn eval(exp0: &ast::Exp, context: &DataStructOfExpData) -> ExpData {
    let n = context.n;
    let repeat_time = context.repeat_time;
    match exp0 {
        ast::Exp::Number {num} => ExpData::from_elem(*num as f64, n, repeat_time),
        ast::Exp::Variable { name } => {
            assert_eq!(name, &"t".to_string());
            context.get_data_by_name_id(name, 0).unwrap().clone()
        }
        ast::Exp::VariableId { name, id } => {
            context.get_data_by_name_id(name, *id).unwrap().clone()
        }
        ast::Exp::UnaryExp { op: ast::UnaryOp::Neg, ref exp } => -eval(&*exp, context),
        ast::Exp::UnaryExp { op: ast::UnaryOp::Diff, ref exp } => eval(&*exp, context).diff_tau(),
        ast::Exp::BinaryExp { op: ast::BinaryOp::Add, ref left, ref right } => eval(&*left, context) + eval(&*right, context),
        ast::Exp::BinaryExp { op: ast::BinaryOp::Sub, ref left, ref right } => eval(&*left, context) - eval(&*right, context),
        ast::Exp::BinaryExp { op: ast::BinaryOp::Mul, ref left, ref right } => eval(&*left, context) * eval(&*right, context),
        ast::Exp::BinaryExp { op: ast::BinaryOp::Div, ref left, ref right } => eval(&*left, context) / eval(&*right, context),
        ast::Exp::BinaryExp { op: ast::BinaryOp::Pow, ref left, ref right } => eval(&*left, context).pow(&eval(&*right, context)),
        ast::Exp::DiffExp { ref left, ref right, ord} =>
            eval(&*left, context).diff_n(&eval(&*right, context), *ord as usize),
        
    }
}
