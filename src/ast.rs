use std::fmt;

pub enum Func {
    Sum,
    Prod,
    Forall,
}

pub enum BinaryOp {
    Add, Sub, Mul, Div, Pow
}
impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Pow => write!(f, "**"),
        }
    }
}
pub enum UnaryOp {
    Neg,
    Diff,
}
impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Diff => write!(f, "D"),
        }
    }
}

pub enum Exp {
    Number {num: i32},
    Variable {name: String},
    VariableId {name: String, id: i32},
    UnaryExp {op: UnaryOp, exp: Box<Exp>},
    BinaryExp {left: Box<Exp>, op: BinaryOp, right: Box<Exp>},
    DiffExp {left: Box<Exp>, right: Box<Exp>, ord: i32},
}
impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number {num} => write!(f, "{}", num),
            Exp::Variable {name} => write!(f, "{}", name),
            Exp::VariableId {name, id} => write!(f, "{}[{}]", name, id),
            Exp::UnaryExp {op, exp} =>
                match op {
                    UnaryOp::Neg => write!(f, "-{}", exp),
                    UnaryOp::Diff => write!(f, "D.{}", exp),
                }
            Exp::BinaryExp {left, op, right} => write!(f, "({} {} {})", left, op, right),
            Exp::DiffExp {left, right, ord} => 
                match ord {
                    1 => write!(f, "D[{}]/D[{}]", left, right),
                    _ => write!(f, "D^{}[{}]/D[{}]^{}", ord, left, right,ord),
                }
        }
    }
}
