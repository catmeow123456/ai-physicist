use crate::ast::{
    AtomExp, Exp, Proposition
};

pub trait Complexity<T: PartialOrd> {
    fn complexity(&self) -> T;
}

impl Complexity<i32> for AtomExp {
    fn complexity(&self) -> i32 {
        match self {
            AtomExp::Variable { name:_ } => 1,
            AtomExp::VariableIds { name:_, ids } => ids.len() as i32,
        }
    }
}

impl Complexity<i32> for Exp {
    fn complexity(&self) -> i32 {
        match self {
            Exp::Atom { atom } => atom.complexity(),
            Exp::Number { num: _ } => 1,
            Exp::BinaryExp { left, op: _, right } => left.complexity() + right.complexity(),
            Exp::UnaryExp { op: _, exp } => exp.complexity() + 1,
            Exp::DiffExp { left, right, ord } => left.complexity() + right.complexity() + ord,
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