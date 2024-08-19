use crate::ast::{
    AtomExp, Exp, Proposition, BinaryOp
};

pub trait Complexity<T: PartialOrd> {
    fn complexity(&self) -> T;
}

impl Complexity<i32> for AtomExp {
    fn complexity(&self) -> i32 {
        match self {
            AtomExp::Variable { name:_ } => 10,
            AtomExp::VariableIds { name:_, ids } => ids.len() as i32 * 10,
        }
    }
}

impl Complexity<i32> for BinaryOp {
    fn complexity(&self) -> i32 {
        match self {
            BinaryOp::Add => 2,
            BinaryOp::Sub => 2,
            BinaryOp::Mul => 4,
            BinaryOp::Div => 5,
            BinaryOp::Pow => 6,
        }
    }
}

impl Complexity<i32> for Exp {
    fn complexity(&self) -> i32 {
        match self {
            Exp::Atom { atom } => atom.complexity(),
            Exp::Number { num: _ } => 5,
            Exp::BinaryExp { left, op, right } => left.complexity() + right.complexity() + op.complexity(),
            Exp::UnaryExp { op: _, exp } => exp.complexity() + 5,
            Exp::DiffExp { left, right, ord } => left.complexity() + right.complexity() + ord * 8,
            Exp::ExpWithMeasureType { exp, measuretype: _ } => exp.complexity(),
        }
    }
}

impl Complexity<i32> for Proposition {
    fn complexity(&self) -> i32 {
        match self {
            Proposition::IsConserved { exp } => {
                exp.complexity()
            }
            Proposition::IsZero { exp } => {
                exp.complexity()
            }
            Proposition::Eq { left, right } => {
                left.complexity() + right.complexity()
            }
            Proposition::Not { prop } => {
                prop.complexity()
            }
        }
    }
}