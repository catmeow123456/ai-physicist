use pyo3::prelude::*;
use crate::experiments::expdata::ExpData;
use crate::experiments::expstructure::DataStruct;
use crate::ast::Exp;

#[pyfunction]
pub fn search_relations(fn_list: &DataStruct) -> Vec<(Exp, ExpData)> {
    let mut list: Vec<(Exp, ExpData)> = vec![];
    for ((data, id), value) in fn_list.iter() {
        list.push((Exp::VariableId { name: data.name().clone(), id: *id }, value.clone()));
    }
    search_relations_aux(&list)
}

fn search_relations_aux(list: &Vec<(Exp, ExpData)>) -> Vec<(Exp, ExpData)> {
    for (id, valuei) in list.iter() {
        for (jd, valuej) in list.iter() {
            // if id 
            unimplemented!()
        }
    }
    unimplemented!()
}