use crate::r;
use std::fmt::{self};
use std::collections::{HashMap, HashSet};
use pyo3::prelude::*;
use crate::complexity::Complexity;

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum Func {
    Sum,
    Prod,
    Forall,
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Proposition {
    IsConserved {exp: Box<Exp>},
    IsZero {exp: Box<Exp>},
    Eq {left: Box<Exp>, right: Box<Exp>},
    Not {prop: Box<Proposition>},
}
#[pymethods]
impl Proposition {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AtomExp {
    Variable {name: String},
    VariableIds {name: String, ids: Vec<i32>},
}


#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Exp {
    // ExpConfig -> MeasureData -> ExpData
    Number {num: i32},
    Atom {atom: Box<AtomExp>},
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
    fn unwrap_atom(&self) -> AtomExp {
        match self {
            Exp::Atom {atom} => *atom.clone(),
            _ => panic!("Error: unwrap_atom failed"),
        }
    }
    #[getter]
    fn get_complexity(&self) -> i32 {
        self.complexity()
    }
    #[staticmethod]
    pub fn new_variable(name: String) -> Self {
        Exp::Atom {atom: Box::new(AtomExp::new_variable(name))}
    }
    #[staticmethod]
    pub fn get_t() -> Self {
        Exp::Atom {atom: Box::new(AtomExp::get_t()) }
    }
    #[staticmethod]
    pub fn new_variable_ids(name: String, ids: Vec<i32>) -> Self {
        if ids.len() == 0 {
            Exp::Atom {atom: Box::new(AtomExp::Variable {name})}
        } else {
            Exp::Atom {atom: Box::new(AtomExp::VariableIds {name, ids})}
        }
    }
    pub fn subst(&self, oid: i32, nid: i32) -> Self{
        match self {
            Exp::Number { num } =>
                Exp::Number {num: *num},
            Exp::Atom { atom } =>
                Exp::Atom {atom: Box::new(atom.subst(oid, nid))},
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
            Exp::Number { num: _ } => HashSet::new(),
            Exp::Atom { atom } => atom.get_allids(),
            Exp::UnaryExp { op:_, exp} => exp.get_allids(),
            Exp::BinaryExp { left, op:_, right} => {
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
        }
    }
}
impl Exp {
    pub fn substs(&self, sub_dict: &HashMap<i32, i32>) -> Self {
        match self {
            Exp::Number { num } =>
                Exp::Number {num: *num},
            Exp::Atom { atom } =>
                Exp::Atom {atom: Box::new(atom.substs(sub_dict.clone()))},
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
#[pymethods]
impl SExp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    pub fn get_expconfig(&self) -> IExpConfig {
        match self {
            SExp::Mk {expconfig, exp:_} => (**expconfig).clone(),
        }
    }
    pub fn get_objtype_id_map(&self) -> HashMap<String, HashSet<i32>> {
        self.get_expconfig().get_objtype_id_map()
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum TExp {
    // {ObjStructure} -> ExpConfig -> MeasureData -> ExpData
    Mk0 {exp: Box<Exp>},
    // Mk {objtype: String, exp: Box<Exp>, id: i32},
    Mksucc {objtype: String, texp: Box<TExp>, id: i32},
}
impl TExp {
    fn _subst(&self, idlist: Vec<i32>, sub_dict: HashMap<i32, i32>) -> Exp {
        match self {
            TExp::Mk0 {exp} => {
                let ref exp = **exp;
                assert_eq!(idlist.len(), 0);
                exp.substs(&sub_dict)
            }
            TExp::Mksucc {objtype: _, texp, id} => {
                let mut sub_dict = sub_dict;
                let mut idlist = idlist;
                let nid = idlist.pop().unwrap();
                let ref texp = **texp;
                sub_dict.insert(*id, nid);
                // println!("debug {} {}", id, nid);
                texp._subst(idlist, sub_dict)
            }
        }
    }
    pub fn substs(&self, sub_dict: &HashMap<i32, i32>) -> Exp {
        match self {
            TExp::Mk0 { exp } => exp.substs(&sub_dict),
            TExp::Mksucc { objtype:_, texp, id:_ } => texp.substs(sub_dict)
        }
    }
    pub fn to_atomexp(&self, ids: Vec<i32>) -> AtomExp {
        let x = self.subst(ids);
        match x {
            Exp::Atom {atom} => *atom,
            _ => panic!("Error: TExp to AtomExp Failed"),
        }
    }
}
#[pymethods]
impl TExp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    pub fn subst(&self, idlist: Vec<i32>) -> Exp {
        self._subst(idlist, HashMap::new())
    }
    fn subst_by_dict(&self, sub_dict: HashMap<i32, i32>) -> Exp {
        self.substs(&sub_dict)
    }
    pub fn get_exp(&self) -> Exp {
        match self {
            TExp::Mk0 {exp} => (**exp).clone(),
            TExp::Mksucc {objtype: _, texp, id: _} => texp.get_exp(),
        }
    }
    pub fn get_preids(&self) -> Vec<i32> {
        match self {
            TExp::Mk0 {exp:_} => vec![],
            TExp::Mksucc {objtype: _, texp, id} => {
                let mut s = texp.get_preids();
                s.push(*id);
                s
            },
        }
    }
    pub fn get_objtype_id_map(&self) -> HashMap<String, HashSet<i32>> {
        match self {
            TExp::Mk0 {exp:_} => HashMap::new(),
            TExp::Mksucc {objtype, texp, id} => {
                let mut res = texp.get_objtype_id_map();
                let res_objtype = res.entry(objtype.clone()).or_insert(HashSet::new());
                res_objtype.insert(*id);
                res
            },
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
    fn get_objtype_id_map(&self) -> HashMap<String, HashSet<i32>> {
        match self {
            IExpConfig::From {name: _} => HashMap::new(),
            IExpConfig::Mk {objtype, expconfig, id} => {
                let mut res = expconfig.get_objtype_id_map();
                let res_objtype = res.entry(objtype.clone()).or_insert(HashSet::new());
                res_objtype.insert(*id);
                res
            },
            IExpConfig::Mkfix {object:_, expconfig, id:_} => {
                expconfig.get_objtype_id_map()
            },
        }
    }
    fn get_preids(&self) -> Vec<i32> {
        match self {
            IExpConfig::From {name: _} => vec![],
            IExpConfig::Mk {objtype: _, expconfig, id} => {
                let mut s = expconfig.get_preids();
                s.push(*id);
                s
            },
            IExpConfig::Mkfix {object:_, expconfig, id:_} => {
                expconfig.get_preids()
            },
        }
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum ObjAttrExp {
    // {ObjStructure} -> ExpData  与 MeasureData 无关，与 ObjStructure 有关
    From { sexp: Box<SExp> },
}
impl ObjAttrExp {
    pub fn get_sexp(&self) -> SExp {
        match self {
            ObjAttrExp::From {sexp} => (**sexp).clone(),
        }
    }
    pub fn get_objtype_id_map(&self) -> HashMap<String, HashSet<i32>> {
        self.get_sexp().get_objtype_id_map()
    }
    pub fn get_preids(&self) -> Vec<i32> {
        self.get_sexp().get_expconfig().get_preids()
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Expression {
    Exp {exp: Box<Exp>},
    SExp {sexp: Box<SExp>},
    TExp {texp: Box<TExp>},
    ObjAttrExp {objattrexp: Box<ObjAttrExp>},
    Proposition {prop: Box<Proposition>},
}

#[pymethods]
impl AtomExp {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[staticmethod]
    pub fn new_variable_ids(name: String, ids: Vec<i32>) -> Self {
        if ids.len() == 0 {
            AtomExp::Variable {name}
        } else {
            AtomExp::VariableIds {name, ids}
        }
    }
    #[staticmethod]
    pub fn new_variable(name: String) -> Self {
        AtomExp::Variable {name}
    }
    #[staticmethod]
    pub fn get_t() -> Self {
        AtomExp::VariableIds { name: r!("t"), ids: vec![0] }
    }
    pub fn get_name(&self) -> String {
        match self {
            AtomExp::Variable {name} => name.clone(),
            AtomExp::VariableIds {name, ids:_} => name.clone(),
        }
    }
    pub fn get_vec_ids(&self) -> Vec<i32> {
        match self {
            AtomExp::Variable {name:_} => vec![],
            AtomExp::VariableIds {name:_, ids} => ids.clone(),
        }
    }
    pub fn get_allids(&self) -> HashSet<i32> {
        match self {
            AtomExp::Variable {name:_} => HashSet::new(),
            AtomExp::VariableIds {name:_ , ids} => {
                let mut res = HashSet::new();
                for id in ids.iter() {
                    res.insert(*id);
                }
                res
            },
        }
    }
    pub fn subst(&self, oid: i32, nid: i32) -> Self {
        match self {
            AtomExp::Variable {name} => AtomExp::Variable {name: name.clone()},
            AtomExp::VariableIds {name, ids} => {
                let ids = ids.clone();
                let mut res = Vec::new();
                for id in ids.iter() {
                    if *id == oid {
                        res.push(nid);
                    } else {
                        res.push(*id);
                    }
                }
                AtomExp::VariableIds {name: name.clone(), ids: res}
            }
        }
    }
    pub fn substs(&self, sub_dict: HashMap<i32, i32>) -> Self {
        match self {
            AtomExp::Variable {name} => AtomExp::Variable {name: name.clone()},
            AtomExp::VariableIds {name, ids} => {
                let ids = ids.clone();
                let mut res = Vec::new();
                for id in ids.iter() {
                    match sub_dict.get(id) {
                        Some(nid) => res.push(*nid),
                        None => res.push(*id),
                    }
                }
                AtomExp::VariableIds {name: name.clone(), ids: res}
            }
        }
    }
}
