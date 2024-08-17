use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::exprcharacter::{KeyState, KeyValue, KeyValueHashed};
use crate::experiments::objects::obj::ObjType;
use crate::r;
use crate::ast::{Proposition, UnaryOp, BinaryOp, AtomExp, Exp, SExp, TExp, ObjAttrExp, IExpConfig, Expression, MeasureType};
use crate::experiments::simulation::{
    oscillation::struct_oscillation,
    collision::struct_collision,
    motion::struct_motion,
    motion0::struct_motion0,
};
use crate::experiments::{
    expdata::{ExpData, Diff},
    expstructure::{ExpStructure, Objstructure},
};

#[pyclass]
pub struct Knowledge {
    experiments: HashMap<String, ExpStructure>,
    pub concepts: HashMap<String, Expression>,
    // concepts only support two kinds of Expression: ObjAttrExp and TExp.
    objects: HashMap<String, Objstructure>,
    // key is used to calculate the Concept's Expression characteristic value.
    // to classify wheather two Concepts are the same.
    // for example, 1 + x and 2x + 1 - x are the same (under simplification).
    // for example, v[1] - x[2] and v[2] - x[1] are the same (under permutation of index).
    pub key: KeyState,
    // conclusions:
    conclusions: HashMap<String, Proposition>,
}

#[pymethods]
impl Knowledge {
    #[new]
    pub fn new() -> Self {
        Self {
            experiments: HashMap::new(),
            concepts: HashMap::new(),
            objects: HashMap::new(),
            key: KeyState::new(None),
            conclusions: HashMap::new(),
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        Self {
            experiments: HashMap::from([
                (r!("oscillation"), struct_oscillation()),
                (r!("collision"), struct_collision()),
                (r!("motion"), struct_motion()),
                (r!("motion0"), struct_motion0()),
            ]),
            concepts: HashMap::new(),
            objects: HashMap::new(),
            key: KeyState::new(None),
            conclusions: HashMap::new(),
        }
    }
    fn list_experiments(&self) {
        for (name, _) in self.experiments.iter() {
            println!("{}", name);
        }
    }
    fn list_concepts(&self) {
        // enumerate self.concepts by order of name
        let mut vec: Vec<_> = self.concepts.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        for (name, expression) in vec.iter() {
            println!("{}: {}", name, expression);
        }
    }
    fn list_conclusions(&self) {
        // enumerate self.concepts by order of name
        let mut vec: Vec<_> = self.conclusions.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        for (name, prop) in vec.iter() {
            println!("{}: {}", name, prop);
        }
    }
    #[inline]
    fn fetch_experiments(&self) -> Vec<String> {
        let mut res = vec![];
        for (name, _) in self.experiments.iter() {
            res.push(name.clone());
        }
        res
    }
    #[inline]
    fn fetch_concepts(&self) -> HashMap<String, Expression> {
        self.concepts.clone()
    }
    #[inline]
    fn fetch_concept_by_name(&self, name: String) -> Expression {
        self.concepts.get(&name).unwrap().clone()
    }
    #[inline]
    fn fetch_conclusions(&self) -> HashMap<String, Proposition> {
        self.conclusions.clone()
    }
    #[inline]
    fn fetch_conclusion_by_name(&self, name: String) -> Proposition {
        self.conclusions.get(&name).unwrap().clone()
    }
    #[inline]
    fn register_object(&mut self, name: String, obj: Objstructure) {
        self.objects.insert(name, obj);
    }
    #[inline]
    fn register_experiment(&mut self, name: String, exp: ExpStructure) {
        self.experiments.insert(name, exp);
    }
    #[inline]
    fn register_expression(&mut self, name: String, exp: Expression) {
        match &exp {
            Expression::TExp { texp } => {
                let (kv, kvh, subs_dict) = self.eval_concept_keyvaluehashed(&exp);
                if kvh.is_none() || kvh.is_const() || self.key.contains_key(&kvh) {
                    return;
                }
                let ids: Vec<_> = texp.get_preids().iter().map(|x| *subs_dict.get(&x).unwrap()).collect();
                let atom = AtomExp::new_variable_ids(name.clone(), ids);
                self.key.insert(atom, kv, kvh);
            },
            _ => ()
        };
        self.concepts.insert(name, exp);
    }
    #[inline]
    fn register_conclusion(&mut self, name: String, prop: Proposition) {
        self.conclusions.insert(name, prop);
    }
    #[inline]
    fn remove_conclusion(&mut self, name: String) {
        self.conclusions.remove(&name);
    }
    #[inline]
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
                let sexp = sexp.as_ref();
                match sexp {
                    SExp::Mk { expconfig, exp } => {
                        let expconfig = expconfig.as_ref();
                        let mut data = self.get_expstructure(expconfig, objsettings);
                        let exp = exp.as_ref();
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
                let exp = exp.as_ref();
                self._eval(exp, context)
            }
            _ => {
                self._eval(exp0, context)
            }
        }
    }
    #[inline]
    pub fn eval_expr_key(&mut self, exp: &Expression) -> KeyValueHashed {
        self.eval_concept_keyvaluehashed(exp).1
    }
    pub fn generalize_sexp(&self, sexp: &SExp) -> TExp {
        match sexp {
            SExp::Mk { expconfig, exp } => {
                self.generalize(exp.as_ref(), expconfig.get_expname())
            }
        }
    }
    pub fn generalize(&self, expr: &Exp, exp_name: String) -> TExp {
        let ref expstructure = *self.experiments.get(&exp_name).unwrap();
        let mut vec = vec![];
        for item in expr.get_allids() {vec.push(item);}
        let n = vec.len();
        assert!(n > 0);
        let perm = (1..(n+1)).permutations(n);
        let mut nexp = expr.clone();
        let mut nexp_subs_dict: HashMap<i32, i32> = HashMap::new();
        for p in perm {
            let mut subst_dict: HashMap<i32, i32> = HashMap::new();
            for (i, j) in vec.iter().zip(p) {
                subst_dict.insert(*i, j as i32);
            }
            let new_exp = expr.substs(&subst_dict);
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
        for i in 1..(n+1) {
            let objtype = id_objtype_map.get(&(i as i32)).unwrap();
            // println!("--({}->{}), ", i, objtype);
            texp_res = TExp::Mksucc {
                objtype: objtype.clone(),
                texp: Box::new(texp_res),
                id: i as i32,
            };
        }
        texp_res
    }
    pub fn specialize(&self, texp: &TExp, exp_name: String) -> Vec<Exp> {
        let vec_map = self._get_all_possible_map(&texp.get_objtype_id_map(), exp_name);
        let mut res: Vec<Exp> = vec![];
        for dict in vec_map.iter() {
            let new_exp = texp.substs(dict);
            res.push(new_exp);
        }
        res
    }
    pub fn specialize_concept(&self, concept_name: String, exp_name: String) -> Vec<AtomExp> {
        let concept = self.concepts.get(&concept_name).unwrap();
        match concept {
            Expression::ObjAttrExp { objattrexp } => {
                let vec_map = self._get_all_possible_map(&objattrexp.get_objtype_id_map(), exp_name);
                let preids = objattrexp.get_preids();
                let mut exp_list = vec![];
                for dict in vec_map.iter() {
                    let mut ids = vec![];
                    for id in preids.iter() {
                        ids.push(*dict.get(id).unwrap());
                    }
                    exp_list.push(AtomExp::new_variable_ids(concept_name.clone(), ids));
                }
                unimplemented!()
            }
            Expression::TExp { texp } => {
                let vec_map = self._get_all_possible_map(&texp.get_objtype_id_map(), exp_name);
                let preids = texp.get_preids();
                let mut exp_list = vec![];
                for dict in vec_map.iter() {
                    let mut ids = vec![];
                    for id in preids.iter() {
                        ids.push(*dict.get(id).unwrap());
                    }
                    exp_list.push(AtomExp::new_variable_ids(concept_name.clone(), ids));
                }
                exp_list
            }
            _ => unimplemented!()
        }
    }
    #[inline]
    fn eval_exp_keyvalue(&mut self, exp: &Exp) -> KeyValue {
        self.eval_keyvalue(exp)
    }
    #[inline]
    fn eval_exp_keyvaluehashed(&mut self, exp: &Exp) -> KeyValueHashed {
        self.eval_keyvalue(exp).to_hashed()
    }
}
impl Knowledge {
    fn _get_all_possible_map(&self, objtype_id_map: &HashMap<String, HashSet<i32>>, exp_name: String) -> Vec<HashMap<i32, i32>> {
        let ref expstructure = *self.experiments.get(&exp_name).unwrap();
        let mut vec_map: Vec<HashMap<i32, i32>> = vec![];
        vec_map.push(HashMap::new());
        for (objtype, ids) in objtype_id_map.iter() {
            let choose_ids = expstructure.get_obj_ids(ObjType::from_str(objtype).unwrap());
            let perm = choose_ids.iter().permutations(ids.len());
            let mut vec_map_of_objtype = vec![];
            for p in perm {
                let dict: HashMap<i32, i32> = ids.iter().zip(p).map(|(a, b)| (*a, *b as i32)).collect();
                vec_map_of_objtype.push(dict);
            }
            let mut vec_map_new: Vec<HashMap<i32, i32>> = vec![];
            for dict in vec_map.iter() {
                for dict_objtype in vec_map_of_objtype.iter() {
                    let mut new_dict = dict.clone();
                    new_dict.extend(dict_objtype.clone());
                    vec_map_new.push(new_dict);
                }
            }
            vec_map = vec_map_new;
        }
        vec_map
    }
}

impl Knowledge {
    fn _eval(&self, exp0: &Exp, context: &mut ExpStructure) -> ExpData {
        assert!(!context.expdata_is_none());
        let data = context.get_ref_expdata();
        let n = data.measuretype.n();
        let repeat_time = data.measuretype.repeat_time();
        match exp0 {
            Exp::ExpWithMeasureType { exp: _, measuretype: _ } => {
                panic!("ExpWithMeasureType should be handled in eval");
            }
            Exp::Number { num } => {
                ExpData::from_elem(*num as f64, n, repeat_time)
            }
            Exp::Atom { atom } => {
                let atom = atom.as_ref();
                // println!("atom = {}", atom);
                let expdata = data.get_data().get_data_by_key(atom);
                match expdata {
                    Ok(expdata) => expdata,
                    Err(_) => {
                        // println!("{} {}", atom, atom.get_name());
                        let expr = self.concepts.get(&atom.get_name()).unwrap();
                        match expr {
                            Expression::ObjAttrExp { objattrexp } => {
                                let mut objs = vec![];
                                for id in atom.get_allids().iter() {
                                    objs.push(context.get_obj(*id).clone());
                                }
                                let expdata = self.eval_objattr(objattrexp, objs);
                                context.get_mut_expdata().set_data(atom.clone(), expdata.clone());
                                expdata
                            }
                            Expression::TExp { texp } => {
                                let texp_new = texp.subst(atom.get_vec_ids());
                                let expdata = self._eval(&texp_new, context);
                                context.get_mut_expdata().set_data(atom.clone(), expdata.clone());
                                expdata
                            }
                            _ => unimplemented!()
                        }
                    }
                }
            }
            Exp::UnaryExp { op: UnaryOp::Neg, ref exp } => -self._eval(&*exp, context),
            Exp::UnaryExp { op: UnaryOp::Diff, ref exp } => self._eval(&*exp, context).diff_tau(),
            Exp::BinaryExp { op, ref left, ref right } => 
                apply_binary_op(op, &self._eval(&*left, context), &self._eval(&*right, context)).unwrap(),
            Exp::DiffExp { ref left, ref right, ord} =>
                (&self._eval(&*left, context)).diff_n(&self._eval(&*right, context), *ord as usize),
        }
    }
}

pub fn apply_binary_op(op: &BinaryOp, valuei: &ExpData, valuej: &ExpData) -> Option<ExpData> {
    match op {
        BinaryOp::Add => Some(valuei + valuej),
        BinaryOp::Sub => Some(valuei - valuej),
        BinaryOp::Mul => Some(valuei * valuej),
        BinaryOp::Div => Some(valuei / valuej),
        BinaryOp::Pow => Some(valuei.pow(valuej)),
    }
}