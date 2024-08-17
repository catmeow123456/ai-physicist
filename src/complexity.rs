use crate::ast::Exp;

pub trait Complexity<T: PartialOrd> {
    fn complexity(&self) -> T;
}

impl Complexity<i32> for Exp {
    fn complexity(&self) -> i32 {
        match self {
            Exp::Atom { atom: _ } => 2,
            Exp::Number { num: _ } => 1,
            Exp::BinaryExp { left, op: _, right } => left.complexity() + right.complexity() + 1,
            Exp::UnaryExp { op: _, exp } => exp.complexity() + 1,
            Exp::DiffExp { left, right, ord } => left.complexity() + right.complexity() + ord,
            Exp::ExpWithMeasureType { exp, measuretype: _ } => exp.complexity(),
        }
    }
}