use pyo3::prelude::*;
use crate::expdata::expdata::{ExpData, Diff};
use crate::experiments::expstructure::DataStruct;
use crate::ast::{BinaryOp, Exp};
use crate::knowledge::apply_binary_op;

#[pyfunction]
pub fn search_trivial_relations(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let mut list: Vec<(Exp, ExpData)> = vec![];
    let tdata = fn_list.get_t();
    for (atom, value) in fn_list.iter() {
        if value.is_conserved() {
            list.push((Exp::Atom { atom: Box::new(atom.clone()) }, value.clone()));
        } else {
            let valuedt = value.diff(&tdata);
            if valuedt.is_conserved() {
                let exp = Exp::DiffExp { left: Box::new(Exp::Atom { atom: Box::new(atom.clone()) }), right: Box::new(Exp::get_t()), ord: 1 };
                list.push((exp, valuedt));
            }
        }
    }
    list
}

#[pyfunction]
pub fn search_relations_ver2(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let ref origin_list = fn_list.iter().collect::<Vec<_>>();
    let mut list: Vec<(Exp, ExpData)> = vec![];
    for (atom, value) in origin_list {
        if !value.is_conserved() {
            list.push((Exp::Atom { atom: Box::new((*atom).clone()) }, (*value).clone()));
        }
    }
    for id1 in 0..origin_list.len() {
        for id2 in 0..id1 {
            if id1 == id2 {
                continue;
            }
            let (atom1, value1) = origin_list[id1];
            let (atom2, value2) = origin_list[id2];
            if value1.is_err() || value2.is_err() {
                continue;
            }
            if value1.is_conserved() && value2.is_conserved() {
                continue;
            }
            let value = value1 * value2;
            if value.is_normal() {
                let exp = Exp::BinaryExp {
                    left: Box::new(Exp::Atom { atom: Box::new(atom1.clone()) }),
                    right: Box::new(Exp::Atom { atom: Box::new(atom2.clone()) }),
                    op: BinaryOp::Mul
                };
                list.push((exp, value));
            }
        }
    }
    search_relations_aux(&list)
}

#[pyfunction]
pub fn search_relations(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let mut list: Vec<(Exp, ExpData)> = vec![];
    for (atom, value) in fn_list.iter() {
        if !value.is_conserved() {
            list.push((Exp::Atom { atom: Box::new(atom.clone()) }, value.clone()));
        }
    }
    search_relations_aux(&list)
}

fn search_relations_aux(list: &Vec<(Exp, ExpData)>) -> Vec<(Exp, ExpData)> {
    let mut relation_list = vec![];
    for i in 0..list.len() {
        for j in 0..list.len() {
            if i == j {
                continue;
            }
            let (ref id, ref valuei) = list[i];
            let (ref jd, ref valuej) = list[j];
            if valuei.is_err() || valuej.is_err() {
                continue;
            }
            if j < i {
                for op in vec![BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul] {
                    let value = apply_binary_op(&op, valuei, valuej);
                    if value.is_conserved() {
                        let exp = Exp::BinaryExp {
                            left: Box::new(id.clone()),
                            right: Box::new(jd.clone()),
                            op: op.clone()
                        };
                        relation_list.push((exp, value));
                    }
                }
            }
            let value = apply_binary_op(&BinaryOp::Div, valuei, valuej);
            if value.is_conserved() {
                let exp = Exp::BinaryExp {
                    left: Box::new(id.clone()),
                    right: Box::new(jd.clone()),
                    op: BinaryOp::Div
                };
                relation_list.push((exp, value));
            }
            let value = valuei.diff(valuej);
            // println!("diff value: {}", value);
            if value.is_const() { // conserved but not zero
                let exp = Exp::DiffExp { left: Box::new(id.clone()), right: Box::new(jd.clone()), ord: 1 };
                relation_list.push((exp, value));
            }
        }
    }
    relation_list
}