use std::fmt;
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum Func {
    Sum,
    Prod,
    Forall,
}

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
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


#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
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

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub struct MeasureType {
    pub t_end: f64,
    pub n:usize,
    pub repeat_time: usize,
    pub error: f64,
}
impl MeasureType {
    pub fn new(t_end: f64, n: i32, repeat_time: i32, error: f64) -> Self {
        Self {
            t_end,
            n: n as usize,
            repeat_time: repeat_time as usize,
            error,
        }
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Exp {
    // ExpConfig -> MeasureData -> ExpData
    Number {num: i32},
    Variable {name: String},
    VariableId {name: String, id: i32},
    UnaryExp {op: UnaryOp, exp: Box<Exp>},
    BinaryExp {left: Box<Exp>, op: BinaryOp, right: Box<Exp>},
    DiffExp {left: Box<Exp>, right: Box<Exp>, ord: i32},
    ExpWithMeasureType {exp: Box<Exp>, measuretype: Box<MeasureType>},
}
impl Exp {
    pub fn subst(&self, oid: i32, nid: i32) -> Self{
        match self {
            Exp::Number {num} => Exp::Number {num: *num},
            Exp::Variable {name} => Exp::Variable {name: name.clone()},
            Exp::VariableId {name, id} => {
                if *id == oid {
                    Exp::VariableId {name: name.clone(), id: nid}
                } else {
                    self.clone()
                }
            }
            Exp::UnaryExp {op, exp} =>
                Exp::UnaryExp {op: op.clone(), exp: Box::new(exp.subst(oid, nid))},
            Exp::BinaryExp {left, op, right} =>
                Exp::BinaryExp {left: Box::new(left.subst(oid, nid)), op: op.clone(), right: Box::new(right.subst(oid, nid))},
            Exp::DiffExp {left, right, ord} => Exp::DiffExp {left: Box::new(left.subst(oid, nid)), right: Box::new(right.subst(oid, nid)), ord: *ord},
            Exp::ExpWithMeasureType {exp, measuretype} => Exp::ExpWithMeasureType {exp: Box::new(exp.subst(oid, nid)), measuretype: measuretype.clone()},
        }
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum SExp {
    //  {ObjStructure} -> MeasureData -> ExpData
    Mk {expconfig: Box<IExpConfig>, exp: Box<Exp>},
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum TExp {
    // {ObjStructure} -> ExpConfig -> MeasureData -> ExpData
    Mk {objtype: String, exp: Box<Exp>, id: i32},
}
impl TExp {
    pub fn subst(&self, nid: i32) -> Exp {
        match self {
            TExp::Mk {objtype: _, exp, id} => {
                let ref exp = **exp;
                exp.subst(*id, nid)
            }
        }        
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum IExpConfig {
    // {ObjStructure} -> ExpConfig
    From { name: String },
    Mk { objtype: String, expconfig: Box<IExpConfig>, id: i32},
    Mkfix { object: String, expconfig: Box<IExpConfig>, id: i32},
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum ObjAttrExp {
    // {ObjStructure} -> ExpData  与 MeasureData 无关，与 ObjStructure 有关
    From { sexp: Box<SExp> },
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Expression {
    Exp {exp: Box<Exp>},
    SExp {sexp: Box<SExp>},
    TExp {texp: Box<TExp>},
    ObjAttrExp {objattrexp: Box<ObjAttrExp>},
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
            Exp::ExpWithMeasureType {exp, measuretype} => write!(f, "{} with {}", exp, measuretype),
        }
    }
}
impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Mk {expconfig, exp} => write!(f, "{} |- {}", expconfig, exp),
        }
    }
}
impl fmt::Display for TExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TExp::Mk {objtype, exp, id} => write!(f, "({}->{}) |- {}", id, objtype, exp),
        }
    }
}
impl fmt::Display for IExpConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IExpConfig::From {name} => write!(f, "#{}", name),
            IExpConfig::Mk {objtype, expconfig, id} => write!(f, "{} ({}->{})", expconfig, id, objtype),
            IExpConfig::Mkfix {object, expconfig, id} => write!(f, "{} [{}->{}]", expconfig, id, object),
        }
    }
}
impl fmt::Display for ObjAttrExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjAttrExp::From {sexp} => write!(f, "[{}]", sexp),
        }
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Exp {exp} => write!(f, "{}", exp),
            Expression::SExp {sexp} => write!(f, "{}", sexp),
            Expression::TExp {texp} => write!(f, "{}", texp),
            Expression::ObjAttrExp {objattrexp} => write!(f, "{}", objattrexp),
        }
    }
}
impl fmt::Display for MeasureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[t_end={}, n={}, repeat_time={}, error={}]", self.t_end, self.n, self.repeat_time, self.error)
    }
}