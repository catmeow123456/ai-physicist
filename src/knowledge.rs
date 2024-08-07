use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::HashMap;
use crate::r;
use crate::ast::{UnaryOp, BinaryOp, Exp, SExp, TExp, ObjAttrExp, IExpConfig, Expression, MeasureType};
use crate::experiments::simulation::{
    oscillation::struct_oscillation,
    collision::struct_collision,
    motion::struct_motion,
    motion0::struct_motion0,
};
use crate::experiments::{
    expdata::{ExpData, Diff},
    expstructure::{ExpStructure, Objstructure},
    objects::obj::DATA,
};
#[pyclass]
pub struct Knowledge {
    experiments: HashMap<String, ExpStructure>,
    concepts: HashMap<String, Expression>,
    objects: HashMap<String, Objstructure>,
}

#[pymethods]
impl Knowledge {
    #[new]
    pub fn new() -> Self {
        Self {
            experiments: HashMap::from([
                (r!("oscillation"), struct_oscillation()),
                (r!("collision"), struct_collision()),
                (r!("motion"), struct_motion()),
                (r!("motion0"), struct_motion0()),
            ]),
            concepts: HashMap::new(),
            objects: HashMap::new(),
        }
    }
    fn list_experiments(&self) {
        for (name, _) in self.experiments.iter() {
            println!("{}", name);
        }
    }
    fn fetch_experiments(&self) -> Vec<String> {
        let mut res = vec![];
        for (name, _) in self.experiments.iter() {
            res.push(name.clone());
        }
        res
    }
    fn list_concepts(&self) {
        for (name, expression) in self.concepts.iter() {
            println!("{} {}", name, expression);
        }
    }
    fn fetch_concepts(&self) -> HashMap<String, Expression> {
        self.concepts.clone()
    }
    fn register_object(&mut self, name: String, obj: Objstructure) {
        self.objects.insert(name, obj);
    }
    fn register_experiment(&mut self, name: String, exp: ExpStructure) {
        self.experiments.insert(name, exp);
    }
    fn register_expression(&mut self, name: String, exp: Expression) {
        self.concepts.insert(name, exp);
    }
    fn get_expstruct_pure(&self, name: String) -> ExpStructure {
        self.experiments.get(&name).unwrap().clone()
    }
    fn get_expstructure(&self, expconfig: &IExpConfig, objsettings: Vec<Objstructure> ) -> ExpStructure {
        match expconfig {
            IExpConfig::From { name } => {
                assert_eq!(objsettings.len(), 0);
                let mut exp = (*self.experiments.get(name).unwrap()).clone();
                exp.random_sample();
                exp
            }
            IExpConfig::Mk { objtype, expconfig, id } => {
                let mut objsettings = objsettings;
                let obj = objsettings.pop().unwrap();
                assert_eq!(obj.obj_type.to_string(), *objtype);
                let mut exp = self.get_expstructure(expconfig, objsettings);
                exp.set_obj(*id, obj);
                exp
            }
            IExpConfig::Mkfix { object, expconfig, id } => {
                let obj = self.objects.get(object).unwrap();
                let mut exp = self.get_expstructure(expconfig, objsettings);
                exp.set_obj(*id, obj.clone());
                exp
            }
        }
    }
    pub fn eval_objattr(&self, objattrexp: &ObjAttrExp, objsettings: Vec<Objstructure>) -> ExpData {
        match objattrexp {
            ObjAttrExp::From { sexp } => {
                let ref sexp = **sexp;
                match sexp {
                    SExp::Mk { expconfig, exp } => {
                        let ref expconfig = **expconfig;
                        let mut data = self.get_expstructure(expconfig, objsettings);
                        // let data = data.get_expdata(MeasureType::default());
                        let ref exp = **exp;
                        // Ok(eval(exp, data))
                        self.eval(exp, &mut data)
                    }
                }
            }
        }
    }

    pub fn eval(&self, exp0: &Exp, 
                context: &mut ExpStructure) -> ExpData {
        if context.expdata_is_none() {
            match exp0 {
                Exp::ExpWithMeasureType { exp: _, measuretype } => {
                    context.calc_expdata((**measuretype).clone());
                }
                _ => {
                    context.calc_expdata(MeasureType::default());
                }
            }
        }
        assert!(!context.expdata_is_none());
        let data = context.get_ref_expdata();
        match exp0 {
            Exp::ExpWithMeasureType { exp, measuretype } => {
                assert!(**measuretype == data.measuretype);
                let ref exp = **exp;
                self._eval(exp, context)
            }
            _ => {
                self._eval(exp0, context)
            }
        }
    }
    pub fn generalize(&self, sexp: &SExp) -> TExp {
        match sexp {
            SExp::Mk { expconfig, exp } => {
                let ref expname = expconfig.get_expname();
                let ref exp = **exp;
                let ref expstructure = *self.experiments.get(expname).unwrap();
                let mut vec = vec![];
                for item in exp.get_allids() {vec.push(item);}
                let n = vec.len();
                assert!(n > 0);
                let perm = (1..(n+1)).permutations(n);
                let mut nexp = exp.clone();
                let mut nexp_subs_dict: HashMap<i32, i32> = HashMap::new();
                for p in perm {
                    let mut subst_dict: HashMap<i32, i32> = HashMap::new();
                    for (i, j) in vec.iter().zip(p) {
                        subst_dict.insert(*i, j as i32);
                    }
                    let new_exp = exp.substs(&subst_dict);
                    if nexp_subs_dict.is_empty() || format!("{}", new_exp) < format!("{}", nexp) {
                        nexp = new_exp;
                        nexp_subs_dict = subst_dict;
                    }
                }
                // println!("nexp = {}", nexp);
                // println!("nexp_subs_dict = {:?}", nexp_subs_dict);
                let mut id_objtype_map: HashMap<i32, String> = HashMap::new();
                for (i, j) in nexp_subs_dict.iter() {
                    let obj = expstructure.get_obj(*i);
                    id_objtype_map.insert(*j, obj.obj_type.to_string());
                }
                let mut texp_res = TExp::Mk0 { exp: Box::new(nexp) };
                for i in (1..(n+1)).rev() {
                    // println!("--{}, ", i);
                    let objtype = id_objtype_map.get(&(i as i32)).unwrap();
                    texp_res = TExp::Mksucc {
                        objtype: objtype.clone(),
                        texp: Box::new(texp_res),
                        id: i as i32,
                    };
                }
                texp_res
            }
        }
    }
}

impl Knowledge {
    fn _eval(&self, exp0: &Exp, context: &mut ExpStructure) -> ExpData {
        assert!(!context.expdata_is_none());
        let data = context.get_ref_expdata();
        let n = data.measuretype.n();
        let repeat_time = data.measuretype.repeat_time();
        match exp0 {
            Exp::Number {num} => ExpData::from_elem(*num as f64, n, repeat_time),
            Exp::Variable { name } => {
                assert_eq!(name, &"t".to_string());
                data.get_data_by_name_id(name, 0).unwrap().clone()
            }
            Exp::VariableId { name, id } => {
                let res = data.get_data_by_name_id(name, *id);
                match res {
                    Ok(data) => data.clone(),
                    Err(_) => {
                        let obj = context.get_obj(*id);
                        let expr = self.concepts.get(name).unwrap();
                        let d = DATA::Mk { obj: obj.obj_type.clone(), name: name.clone() };
                        match expr {
                            Expression::ObjAttrExp { objattrexp } => {
                                let objsettings = vec![obj.clone()];
                                let expdata = self.eval_objattr(objattrexp, objsettings);
                                context.get_mut_expdata().set_data(d, *id, expdata.clone());
                                expdata
                            }
                            Expression::TExp { texp } => {
                                let expdata = self._eval(&texp.subst(vec![*id]), context);
                                context.get_mut_expdata().set_data(d, *id, expdata.clone());
                                expdata
                            }
                            _ => unimplemented!()
                        }
                    }
                }
            }
            Exp::UnaryExp { op: UnaryOp::Neg, ref exp } => -self._eval(&*exp, context),
            Exp::UnaryExp { op: UnaryOp::Diff, ref exp } => self._eval(&*exp, context).diff_tau(),
            Exp::BinaryExp { op: BinaryOp::Add, ref left, ref right } => self._eval(&*left, context) + self._eval(&*right, context),
            Exp::BinaryExp { op: BinaryOp::Sub, ref left, ref right } => self._eval(&*left, context) - self._eval(&*right, context),
            Exp::BinaryExp { op: BinaryOp::Mul, ref left, ref right } => self._eval(&*left, context) * self._eval(&*right, context),
            Exp::BinaryExp { op: BinaryOp::Div, ref left, ref right } => self._eval(&*left, context) / self._eval(&*right, context),
            Exp::BinaryExp { op: BinaryOp::Pow, ref left, ref right } => self._eval(&*left, context).pow(&self._eval(&*right, context)),
            Exp::DiffExp { ref left, ref right, ord} =>
                (&self._eval(&*left, context)).diff_n(&self._eval(&*right, context), *ord as usize),
            _ => unimplemented!()
        }
    }
}

pub fn apply_binary_op(op: BinaryOp, valuei: &ExpData, valuej: &ExpData) -> Option<ExpData> {
    match op {
        BinaryOp::Add => Some(valuei + valuej),
        BinaryOp::Sub => Some(valuei - valuej),
        BinaryOp::Mul => Some(valuei * valuej),
        BinaryOp::Div => Some(valuei / valuej),
        BinaryOp::Pow => Some(valuei.pow(valuej)),
    }
}