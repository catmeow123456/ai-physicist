use std::fmt::{self, write};
use std::collections::{HashMap, HashSet};
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum Func {
    Sum,
    Prod,
    Forall,
}

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone, Hash)]
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
#[derive(Eq, PartialEq, Clone, Hash)]
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
#[pymethods]
impl Exp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[staticmethod]
    pub fn new_variable(name: String, id: i32) -> Self {
        if id == 0 {
            Exp::Variable {name}
        } else {
            Exp::VariableId {name, id}
        }
    }
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
    pub fn subst_by_dict(&self, sub_dict: HashMap<i32, i32>) -> Self {
        self.substs(&sub_dict)
    }
    pub fn get_allids(&self) -> HashSet<i32> {
        match self {
            Exp::VariableId {name:_, id} => HashSet::from([*id]),
            Exp::UnaryExp {op:_, exp} => exp.get_allids(),
            Exp::BinaryExp {left, op:_, right} => {
                let left = left.get_allids();
                let right = right.get_allids();
                let res: HashSet<i32> = left.union(&right).cloned().collect();
                res
            },
            Exp::DiffExp { left, right, ord:_ } => {
                let left = left.get_allids();
                let right = right.get_allids();
                let res: HashSet<i32> = left.union(&right).cloned().collect();
                res
            },
            Exp::ExpWithMeasureType {exp, measuretype:_} => exp.get_allids(),
            _ => HashSet::new(),
        }
    }
}
impl Exp {
    pub fn substs(&self, sub_dict: &HashMap<i32, i32>) -> Self {
        match self {
            Exp::Number {num} => Exp::Number {num: *num},
            Exp::Variable {name} => Exp::Variable {name: name.clone()},
            Exp::VariableId {name, id} => {
                match sub_dict.get(id) {
                    Some(nid) => Exp::VariableId {name: name.clone(), id: *nid},
                    None => self.clone(),
                }
            }
            Exp::UnaryExp {op, exp} =>
                Exp::UnaryExp {op: op.clone(), exp: Box::new(exp.substs(sub_dict))},
            Exp::BinaryExp {left, op, right} => {
                Exp::BinaryExp {
                    left: Box::new(left.substs(sub_dict)),
                    op: op.clone(),
                    right: Box::new(right.substs(sub_dict))
                }
            },
            Exp::DiffExp {left, right, ord} => {
                Exp::DiffExp {
                    left: Box::new(left.substs(sub_dict)),
                    right: Box::new(right.substs(sub_dict)),
                    ord: *ord
                }
            },
            Exp::ExpWithMeasureType {exp, measuretype} => {
                Exp::ExpWithMeasureType {
                    exp: Box::new(exp.substs(sub_dict)),
                    measuretype: measuretype.clone()
                }
            },
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
    Mk0 {exp: Box<Exp>},
    Mk {objtype: String, exp: Box<Exp>, id: i32},
    Mksucc {objtype: String, texp: Box<TExp>, id: i32},
}
impl TExp {
    fn _subst(&self, idlist: Vec<i32>, sub_dict: HashMap<i32, i32>) -> Exp {
        let mut idlist = idlist;
        let nid = idlist.pop().unwrap();
        let mut sub_dict = sub_dict;
        match self {
            TExp::Mk0 {exp} => {
                let ref exp = **exp;
                assert_eq!(idlist.len(), 0);
                exp.substs(&sub_dict)
            }
            TExp::Mk {objtype: _, exp, id} => {
                let ref exp = **exp;
                assert_eq!(idlist.len(), 0);
                sub_dict.insert(*id, nid);
                exp.substs(&sub_dict)
            }
            TExp::Mksucc {objtype: _, texp, id} => {
                let ref texp = **texp;
                sub_dict.insert(*id, nid);
                texp._subst(idlist, sub_dict)
            }
        }
    }
}
#[pymethods]
impl TExp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    pub fn subst(&self, idlist: Vec<i32>) -> Exp {
        let mut idlist = idlist;
        idlist.reverse();
        self._subst(idlist, HashMap::new())
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
#[pymethods]
impl IExpConfig {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    pub fn get_expname(&self) -> String {
        match self {
            IExpConfig::From {name} => name.clone(),
            IExpConfig::Mk {objtype: _, expconfig, id: _} => expconfig.get_expname(),
            IExpConfig::Mkfix {object: _, expconfig, id: _} => expconfig.get_expname(),
        }
    }
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
            TExp::Mk0 { exp } => write!(f, "|- {}", exp),
            TExp::Mk {objtype, exp, id} => write!(f, "({}->{}) |- {}", id, objtype, exp),
            TExp::Mksucc {objtype, texp, id} => write!(f, "({}->{}) {}", id, objtype, texp),
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