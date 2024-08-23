use pyo3::prelude::*;
use ndarray::Array1;
use super::normaldata::is_conserved;
use super::expdata::ExpData;
use super::constdata::ConstData;

#[pyfunction]
fn is_conserved_const_list(data: Vec<ConstData>) -> bool {
    let mut mean_vec = vec![];
    let mut std_vec = vec![];
    for x in data {
        match x {
            ConstData::Data { mean, std } => {
                mean_vec.push(mean);
                std_vec.push(std);
            },
            ConstData::Exact { value: _ } => {return false},
        };
    }
    // vec to arr
    is_conserved(&Array1::from(mean_vec), &Array1::from(std_vec), None)
}

#[pymodule]
pub fn register_data(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ExpData>()?;
    m.add_class::<ConstData>()?;
    m.add_function(wrap_pyfunction!(is_conserved_const_list, m)?)?;
    Ok(())
}