use crate::ast::{
    AtomExp,
    Exp,
    SExp,
    TExp,
    IExpConfig,
    ObjAttrExp,
    Expression,
    MeasureType,
    Proposition,
    UnaryOp,
};
use std::fmt::{Display, Formatter, Result};

impl Display for AtomExp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AtomExp::Variable {name} => write!(f, "{}", name),
            AtomExp::VariableIds {name, ids} => {
                if ids.len() == 0 {
                    write!(f, "{}", name)
                } else {
                    let str_list = ids.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                    write!(f, "{}[{}]", name, str_list.join(", "))
                }
            },
        }
    }
}
impl Display for Exp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Exp::Number { num } => write!(f, "{}", num),
            Exp::Atom { atom } => write!(f, "{}", atom),
            Exp::UnaryExp { op, exp } =>
                match op {
                    UnaryOp::Neg => write!(f, "-{}", exp),
                    UnaryOp::Diff => write!(f, "D.{}", exp),
                }
            Exp::BinaryExp { left, op, right } => write!(f, "({} {} {})", left, op, right),
            Exp::DiffExp { left, right, ord } => 
                match ord {
                    1 => write!(f, "D[{}]/D[{}]", left, right),
                    _ => write!(f, "D^{}[{}]/D[{}]^{}", ord, left, right,ord),
                }
            Exp::ExpWithMeasureType {exp, measuretype} => write!(f, "{} with {}", exp, measuretype),
        }
    }
}
impl Display for SExp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SExp::Mk {expconfig, exp} => write!(f, "{} |- {}", expconfig, exp),
        }
    }
}

impl TExp {
    fn _aux_print(&self, f: &mut Formatter) -> Result {
        match self {
            TExp::Mk0 {exp:_} => Ok(()),
            TExp::Mksucc {objtype, texp, id} => {
                texp._aux_print(f)?;
                write!(f, "({}->{}) ", id, objtype)
            },
        }
    }
}
impl Display for TExp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self._aux_print(f)?;
        write!(f, "|- {}", self.get_exp())
    }
}
impl Display for IExpConfig {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            IExpConfig::From {name} => write!(f, "#{}", name),
            IExpConfig::Mk {objtype, expconfig, id} => write!(f, "{} ({}->{})", expconfig, id, objtype),
            IExpConfig::Mkfix {object, expconfig, id} => write!(f, "{} [{}->{}]", expconfig, id, object),
        }
    }
}
impl Display for ObjAttrExp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ObjAttrExp::From {sexp} => write!(f, "[{}]", sexp),
        }
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expression::Exp {exp} => write!(f, "{}", exp),
            Expression::SExp {sexp} => write!(f, "{}", sexp),
            Expression::TExp {texp} => write!(f, "{}", texp),
            Expression::ObjAttrExp {objattrexp} => write!(f, "{}", objattrexp),
            Expression::Proposition {prop} => write!(f, "{}", prop),
        }
    }
}
impl Display for MeasureType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[t_end={}, n={}, repeat_time={}, error={}]", self.t_end, self.n, self.repeat_time, self.error)
    }
}
impl Display for Proposition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Proposition::IsConserved {exp} => write!(f, "{} is conserved", exp),
            Proposition::IsZero {exp} => write!(f, "{} is zero", exp),
            Proposition::Eq {left, right} => write!(f, "{} = {}", left, right),
            Proposition::Not {prop} => write!(f, "not ({})", prop),
        }
    }
}