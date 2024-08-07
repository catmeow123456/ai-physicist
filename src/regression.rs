use pyo3::prelude::*;
use crate::experiments::expdata::{ExpData, Diff};
use crate::experiments::expstructure::DataStruct;
use crate::ast::{BinaryOp, Exp};
use crate::knowledge::apply_binary_op;

#[pyfunction]
pub fn search_relations(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let mut list: Vec<(Exp, ExpData)> = vec![];
    for ((data, id), value) in fn_list.iter() {
        if !value.is_conserved() {
            list.push((Exp::new_variable(data.name().clone(), *id), value.clone()));
        }
    }
    search_relations_aux(&list)
}

fn search_relations_aux(list: &Vec<(Exp, ExpData)>) -> Vec<(Exp, ExpData)> {
    let mut relation_list = vec![];
    for i in 0..list.len() {
        for j in 0..i {
            let (ref id, ref valuei) = list[i];
            let (ref jd, ref valuej) = list[j];
            if valuei.badpts.len() >= valuei.n / 4 || valuej.badpts.len() >= valuej.n / 4 {
                continue;
            }
            for op in vec![BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul, BinaryOp::Div] {
                let exp = Exp::BinaryExp {
                    left: Box::new(id.clone()),
                    right: Box::new(jd.clone()),
                    op: op.clone()
                };
                if let Some(value) = apply_binary_op(op.clone(), valuei, valuej) {
                    if value.is_conserved() {
                        relation_list.push((exp, value));
                    }
                }
            }
            if !valuei.is_conserved_piecewise() && !valuej.is_conserved_piecewise() {
                let exp = Exp::DiffExp { left: Box::new(id.clone()), right: Box::new(jd.clone()), ord: 1 };
                let value = valuei.diff(valuej);
                if value.is_conserved() {
                    relation_list.push((exp, value));
                }
            }
        }
    }
    relation_list
}