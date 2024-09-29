use pyo3::prelude::*;
use crate::expdata::expdata::{ExpData, Diff};
use crate::experiments::expstructure::DataStruct;
use crate::ast::{BinaryOp, Exp};
use crate::knowledge::apply_binary_op;

// 对于给定的数据 f(t), ... ， 提取出所有形如 f(t), f'(t) 的守恒量
#[pyfunction]
pub fn search_trivial_relations(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let mut list: Vec<(Exp, ExpData)> = vec![];

    let tdata = if fn_list.has_t() { Some(fn_list.get_t()) } else { None };
    for (atom, value) in fn_list.iter() {
        if value.is_conserved() {
            list.push((Exp::Atom { atom: Box::new(atom.clone()) }, value.clone()));
        } else 
        if let Some(ref tdata) = tdata {
            let valuedt = value.diff(&tdata);
            if valuedt.is_conserved() {
                let exp = Exp::DiffExp { left: Box::new(Exp::Atom { atom: Box::new(atom.clone()) }), right: Box::new(Exp::get_t()), ord: 1 };
                list.push((exp, valuedt));
            }
        }
    }
    list
}


/// 对于给定的数据 f(t), g(t), ... ，
/// 生成所有不超过二次的 （形如 f(t), f(t)g(t) ） 的非守恒单项式
fn gen_monomials(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let ref origin_list = fn_list.iter().collect::<Vec<_>>();
    let mut list: Vec<(Exp, ExpData)> = vec![];
    for (atom, value) in origin_list {
        if !value.is_conserved() {
            list.push((Exp::Atom { atom: Box::new((*atom).clone()) }, (*value).clone()));
        }
    }
    for id1 in 0..origin_list.len() {
        for id2 in 0..(id1+1) {
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
    list
}


// 对于给定的数据 f(t), g(t), ... ，
// 提取出所有形如 h1(t) o h2(t), f1(t)g1(t) o h(t) 或 f1(t)g1(t) o f2(t)g2(t) 的守恒量
// 这里的 o 表示二元运算符， 包括加减乘除和求导
#[pyfunction]
pub fn search_relations_ver2(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let list = gen_monomials(fn_list);
    search_relations_aux(&list)
}


// 对于给定的数据 f(t), g(t), ... ，
// 提取出所有形如
// h1(t) o h2(t), f1(t)g1(t) o h(t), f1(t)g1(t) o f2(t)g2(t),
// f1(t)g1(t)^2 +/- f2(t)g2(t)^2 的守恒量
// 这里的 o 表示二元运算符， 包括加减乘除和求导
#[pyfunction]
pub fn search_relations_ver3(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let ref origin_list = fn_list.iter().collect::<Vec<_>>();
    let mut list = gen_monomials(fn_list);
    let mut result = search_relations_aux(&list);
    for id1 in 0..origin_list.len() {
        for id2 in 0..origin_list.len() {
            let (atom1, value1) = origin_list[id1];
            let (atom2, value2) = origin_list[id2];
            if value1.is_err() || value2.is_err() {
                continue;
            }
            if value1.is_conserved() && value2.is_conserved() {
                continue;
            }
            let value = value1 * &value2.powi(2);
            if value.is_normal() {
                let exp = Exp::BinaryExp {
                    left: Box::new(Exp::Atom { atom: Box::new(atom1.clone()) }),
                    right: Box::new(Exp::BinaryExp {
                            left: Box::new(Exp::Atom { atom: Box::new(atom2.clone()) }),
                            op: BinaryOp::Pow,
                            right: Box::new(Exp::Number { num: 2 })
                        }),
                    op: BinaryOp::Mul
                };
                for (exp0, value0) in list.iter() {
                    for op in vec![BinaryOp::Add, BinaryOp::Sub] {
                        let valuenew = apply_binary_op(&op, value0, &value);
                        if valuenew.is_conserved() {
                            let expnew = Exp::BinaryExp {
                                left: Box::new(exp0.clone()),
                                right: Box::new(exp.clone()),
                                op: op.clone()
                            };
                            if format!("{}", exp) == "(C_16[1] * (C_01[1, 0] ** 2))".to_string() {
                                if format!("{}", exp0) == "(C_16[2] * (C_01[2, 0] ** 2))".to_string() {
                                    println!("!!!!!!!!!! {}", expnew)
                                }
                            }
                            result.push((expnew, valuenew));
                        }
                    }
                }
                list.push((exp, value));
            }
        }
    }
    println!("search_relations_ver3 from list");
    for (exp, _) in list.iter() {
        println!("exp: {}", exp);
    }
    result
}


// 对于给定的数据 f(t), g(t), ... ，
// 提取出所有形如 f(t) o g(t) 的守恒量（这里的 f(t) 和 g(t) 被要求是非守恒的）
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