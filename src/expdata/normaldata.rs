// a data structure that represent n * m arrays
// where n is the number of times of experiments and m is the number of data points

use std::fmt;
use std::f64::NAN;
use pyo3::prelude::*;
use std::ops::{Add, Sub, Mul, Div, Neg};
use ndarray::{s, Array, Array1, Array2, Array3};
use std::collections::HashSet;
use ndarray_linalg::Solve;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use super::constdata::ConstData;
use crate::experiments::expstructure::add_errors;

#[pyclass]
#[derive(Debug, Clone)]
pub struct NormalData {
    pub n: usize,
    pub repeat_time: usize,
    pub data: Array2::<f64>,
    pub badpts: HashSet<usize>,
}

// define the Display trait for ExpData
impl fmt::Display for NormalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExpData.NormalData({}, {}) =\n", self.repeat_time, self.n)?;
        // print the n*m array
        for j in 0..self.repeat_time {
            for i in 0..self.n {
                if self.badpts.contains(&i) {
                    write!(f, "_ ")?;
                    continue;
                }
                write!(f, "{} ", self.data[[j,i]])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[pymethods]
impl NormalData {
    fn __str__(&self) -> String {
        format!("{}", self)
    }
    #[new]
    fn __new__(data: Vec<Vec<f64>>) -> Self {
        let n = data[0].len();
        let repeat_time = data.len();
        let mut arr: Array2::<f64> = Array::zeros((repeat_time, n));
        for i in 0..repeat_time {
            for j in 0..n {
                arr[[i,j]] = data[i][j];
            }
        }
        Self::new(arr)
    }
    #[staticmethod]
    pub fn zero(n: usize, repeat_time: usize) -> Self {
        Self::new(Array2::zeros((repeat_time, n)))
    }
    #[staticmethod]
    pub fn from_elem(mean: f64, std: f64, n: usize, repeat_time: usize) -> Self {
        let arr2 = Array2::from_elem((repeat_time, n), mean);
        Self::new(add_errors(&arr2, std))
    }
    #[staticmethod]
    pub fn from_const_data(content: &ConstData, n: usize, repeat_time: usize) -> Self {
        match content {
            ConstData::Data { mean, std } => {
                let arr = add_errors(&Array1::from_elem(repeat_time, *mean), *std);
                Self::new(Array2::from_shape_fn((repeat_time, n), |(i, _)| arr[i]))
            },
            ConstData::Exact { value } => {
                Self::new(Array2::from_elem((repeat_time, n), *value as f64))
            },
        }
    }
    #[getter]
    fn data(&self) -> PyResult<Vec<Vec<f64>> > {
        let mut res = Vec::new();
        for i in 0..self.repeat_time {
            res.push(self.data.row(i).to_vec())
        }
        Ok(res)
    }
    #[getter]
    fn badpts(&self) -> PyResult<HashSet<usize>> {
        Ok(self.badpts.clone())
    }
    pub fn is_conserved_slice(&self, x: usize, y: usize) -> bool {
        is_conserved(&self.data.slice(s![.., x..y]).mean_axis(ndarray::Axis(0)).unwrap(),
                     &self.data.slice(s![.., x..y]).std_axis(ndarray::Axis(0), 0.0),
                     None)
    }
    pub fn is_conserved_piecewise(&self) -> bool {
        if self.badpts.len() >= self.n / 4 {
            return false
        }
        for (x, y) in self.gen_domain() {
            if !is_conserved(&self.data.slice(s![.., x..y]).mean_axis(ndarray::Axis(0)).unwrap(),
                            &self.data.slice(s![.., x..y]).std_axis(ndarray::Axis(0), 0.0),
                            None) {
                return false
            }
        }
        true
    }
    #[getter]
    pub fn is_conserved(&self) -> bool {
        let mut mean_vec = vec![];
        let mut std_vec = vec![];
        for (x, y) in self.gen_domain() {
            mean_vec.append(&mut self.data.slice(s![.., x..y]).mean_axis(ndarray::Axis(0)).unwrap().to_vec());
            std_vec.append(&mut self.data.slice(s![.., x..y]).std_axis(ndarray::Axis(0), 0.0).to_vec());
        }
        is_conserved(&Array1::from(mean_vec), &Array1::from(std_vec), None)
    }
    #[getter]
    pub fn is_zero(&self) -> bool {
        for (x, y) in self.gen_domain() {
            if !is_zero(&self.data.slice(s![.., x..y]).mean_axis(ndarray::Axis(0)).unwrap(),
                        &self.data.slice(s![.., x..y]).std_axis(ndarray::Axis(0), 0.0),
                        None) {
                return false
            }
        }
        true
    }
    #[inline]
    fn __add__(&self, other: &NormalData) -> PyResult<NormalData> {
        Ok(self + other)
    }
    #[inline]
    fn __sub__(&self, other: &NormalData) -> PyResult<NormalData> {
        Ok(self - other)
    }
    #[inline]
    fn __mul__(&self, other: &NormalData) -> PyResult<NormalData> {
        Ok(self * other)
    }
    #[inline]
    fn __truediv__(&self, other: &NormalData) -> PyResult<NormalData> {
        Ok(self / other)
    }
    #[inline]
    fn __neg__(&self) -> PyResult<NormalData> {
        Ok(-self)
    }
    #[inline]
    fn __powi__(&self, other: i32) -> PyResult<NormalData> {
        Ok(self.powi(other))
    }
    #[inline]
    fn __difftau__(&self) -> PyResult<NormalData> {
        Ok(self.diff_tau())
    }
    #[inline]
    fn __diff__(&self, other: &NormalData) -> PyResult<NormalData> {
        Ok(self.diff_tau() / other.diff_tau())
    }
}


impl NormalData {
    pub fn new(data: Array2::<f64>) -> Self {
        let mut badpts = HashSet::new();
        let n = data.ncols();
        let repeat_time = data.nrows();
        let mut data: Array2::<f64> = data;
        for x in 0..n {
            let mut flag: bool = false;
            for y in 0..repeat_time {
                if data[[y,x]].is_nan() || data[[y,x]].is_infinite(){
                    flag = true;
                    break;
                }
            }
            if flag {
                badpts.insert(x);
                for y in 0..repeat_time {
                    data[[y,x]] = NAN;
                }
            }
        }
        Self {n, repeat_time, data, badpts}
    }

    pub fn gen_domain(&self) -> Vec<(usize,usize)> {
        let mut res = Vec::new();
        let mut last = 0;
        let mut badpts: Vec<usize> = self.badpts.iter().cloned().collect();
        badpts.sort();
        // println!("{:?}", badpts);
        for i in badpts {
            if i > last {
                res.push((last, i))
            }
            last = i + 1;
        }
        if last < self.n {
            res.push((last, self.n))
        }
        res
    }

    pub fn bds_threshold (&self) -> f64 { 5000. }

    pub fn plot_over_t(&self, name: &str, t: &NormalData) {
        // plot the arr
        let mut plot = plotly::Plot::new();
        let repeat_time = self.repeat_time;
        for ith in 0..repeat_time {
            let t = t.data.row(ith).to_vec();
            let x = self.data.row(ith).to_vec();
            let trace = plotly::Scatter::new(t, x);
            plot.add_trace(trace);
        }
        // plot.show();
        plot.write_html(format!("tmp/{}.html", name));
    }
}

impl NormalData {
    pub fn pow(&self, other: &NormalData) -> NormalData {
        let mut res: Array2<f64> = self.data.clone();
        for i in 0..self.n {
            if self.badpts.contains(&i) || other.badpts.contains(&i) {
                for j in 0..self.repeat_time {
                    res[[j,i]] = NAN;
                }
            } else {
                for j in 0..self.repeat_time {
                    res[[j,i]] = self.data[[j,i]].powf(other.data[[j,i]]);
                }
            }
        }
        NormalData::new(res)
    }
    pub fn powi(&self, other: i32) -> NormalData {
        NormalData::new(self.data.mapv(|x| x.powi(other)))
    }
    pub fn diff_tau(&self) -> NormalData {
        let mut data: Array2<f64> = Array::zeros((self.repeat_time, self.n));
        let mut comparedata: Array2<f64> = Array::zeros((self.repeat_time, self.n));
        let mean: Array1<f64> = self.mean();
        let std: Array1<f64> = self.std();
        for x in self.badpts.iter() {
            for j in 0..self.repeat_time {
                data[[j,*x]] = NAN;
            }
        }
        for (x, y) in self.gen_domain() {
            if x as i32 > y as i32 - 10 {
                for i in x..y {
                    for j in 0..self.repeat_time {
                        data[[j,i]] = NAN;
                    }
                }
            } else
            if self.is_conserved_slice(x, y) {
                for i in x..y {
                    for j in 0..self.repeat_time {
                        data[[j,i]] = 0.;
                    }
                }
            } else {
                for j in 0..self.repeat_time {
                    data.row_mut(j).slice_mut(s![x..y]).assign(
                        &npsd(&self.data.slice(s![j, x..y]).to_owned(), 1, 5)
                    );
                    data[[j,x]] = NAN;
                    data[[j,y-1]] = NAN;
                    let d0: Array1<f64> = npsd(&self.data.slice(s![j, x..y;2]).to_owned(), 1, 5);
                    let d1: Array1<f64> = npsd(&self.data.slice(s![j, x+1..y;2]).to_owned(), 1, 5);
                    let mut c0 = 0;
                    let mut c1 = 0;
                    for i in x..y {
                        comparedata[[j,i]] = if (i - x) % 2 == 0 {
                            c0 += 1; d0[c0 - 1] / 2.
                        } else {
                            c1 += 1; d1[c1 - 1] / 2.
                        };
                    }
                }
                for i in x+1..y {
                    let delta: Array1<f64> = &data.slice(s![.., i]) - &comparedata.slice(s![.., i]);
                    if delta.mean().unwrap().powi(2) > self.bds_threshold() * delta.std(0.).powi(2) {
                        for j in 0..self.repeat_time {
                            data[[j,i]] = NAN;
                        }
                    }
                    if (mean[i] - mean[i-1]).abs() < (std[i] + std[i-1]) * 2. {
                        for j in 0..self.repeat_time {
                            data[[j,i]] = NAN;
                            data[[j,i-1]] = NAN;
                        }
                    }
                }
            }
        }
        NormalData::new(data)
    }
    #[inline]
    pub fn mean(&self) -> Array1<f64> {
        self.data.mean_axis(ndarray::Axis(0)).unwrap()
    }
    #[inline]
    pub fn std(&self) -> Array1<f64> {
        self.data.std_axis(ndarray::Axis(0), 0.0)
    }
    #[inline]
    pub fn to_const_data(&self) -> ConstData {
        let domain = self.gen_domain();
        let mut data = Array2::zeros((self.repeat_time, 0));
        for (x, y) in domain {
            data.append(ndarray::Axis(1), self.data.slice(s![.., x..y])).unwrap();
        }
        let (mean, std) = weighted_mean_error(&data.mean_axis(ndarray::Axis(0)).unwrap(), &data.std_axis(ndarray::Axis(0), 0.0));
        ConstData::new(mean, std)
        // ConstData::new(
        //     data.mean().unwrap(),
        //     // data.std(0.0)
        //     // data.mean_axis(ndarray::Axis(1)).unwrap().std(0.0)
        // )
    }
}

impl Neg for NormalData {
    type Output = NormalData;
    #[inline]
    fn neg(self) -> NormalData {
        NormalData::new(-self.data)
    }
}

impl Neg for &NormalData {
    type Output = NormalData;
    #[inline]
    fn neg(self) -> NormalData {
        NormalData::new(-(&self.data))
    }
}

impl Add for NormalData {
    type Output = NormalData;
    #[inline]
    fn add(self, other: NormalData) -> NormalData {
        NormalData::new(self.data + other.data)
    }
}
impl Add for &NormalData {
    type Output = NormalData;
    #[inline]
    fn add(self, other: &NormalData) -> NormalData {
        NormalData::new((&self.data) + (&other.data))
    }
}

impl Sub for NormalData {
    type Output = NormalData;
    #[inline]
    fn sub(self, other: NormalData) -> NormalData {
        NormalData::new(self.data - other.data)
    }
}
impl Sub for &NormalData {
    type Output = NormalData;
    #[inline]
    fn sub(self, other: &NormalData) -> NormalData {
        NormalData::new((&self.data) - (&other.data))
    }
}

impl Mul for NormalData {
    type Output = NormalData;
    #[inline]
    fn mul(self, other: NormalData) -> NormalData {
        NormalData::new(self.data * other.data)
    }
}

impl Mul for &NormalData {
    type Output = NormalData;
    #[inline]
    fn mul(self, other: &NormalData) -> NormalData {
        NormalData::new((&self.data) * (&other.data))
    }
}

impl Div for NormalData {
    type Output = NormalData;
    #[inline]
    fn div(self, other: NormalData) -> NormalData {
        NormalData::new(self.data / other.data)
    }
}

impl Div for &NormalData {
    type Output = NormalData;
    #[inline]
    fn div(self, other: &NormalData) -> NormalData {
        NormalData::new((&self.data) / (&other.data))
    }
}


pub struct NPSCoefficient {
    pub c: Array3<f64>,
    pub r: Array2<f64>,
}

impl NPSCoefficient {
    pub fn new(n: usize) -> Self {
        assert!(n > 0 && n < 20);
        // obtain factorial array
        let mut c: Array1<f64> = Array::ones(n+1);
        for i in 1..n+1 { c[i] = c[i-1] * i as f64; }
        // obtain the transform matrix
        let col: Array1<f64> = Array::linspace(1.0-(n as f64), n as f64-1.0, n*2-1);
        let mut a: Array2<f64> = Array::zeros((n*2-1, n));
        for i in 0..n {
            a.column_mut(i).assign(&col.mapv(|x| x.powi(i as i32) / c[i]));
        }
        let b: Array2<f64> = Array::eye(n);
        let r: Array1<f64> = col.mapv(|x| x.powi(n as i32) / c[n]);
        let mut selfc : Array3<f64> = Array::zeros((n, n, n));
        let mut selfr : Array2<f64> = Array::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                let aa: Array2<f64> = a.slice(s![i..n+i, ..]).t().to_owned();
                let bb: Array1<f64> = b.row(j).to_owned();
                let cc: Array1<f64> = aa.solve_into(bb).unwrap();
                selfc.slice_mut(s![i, j, ..]).assign(&cc);
                selfr[[i,j]] = cc.dot(&r.slice(s![i..n+i]));
            }
        }
        NPSCoefficient {
            c: selfc,
            r: selfr,
        }
    }
}

const NPSC5: [[[f64;5];5];5] = [[
    [0.0, -0.0, -0.0, -0.0, 1.0],
    [0.24999999999999822, -1.3333333333333286, 2.9999999999999925, -3.9999999999999947, 2.083333333333332],
    [0.9166666666666625, -4.666666666666652, 9.499999999999977, -8.66666666666665, 2.916666666666662],
    [1.4999999999999956, -6.999999999999983, 11.999999999999973, -8.99999999999998, 2.4999999999999947],
    [0.9999999999999982, -3.999999999999993, 5.9999999999999885, -3.999999999999992, 0.999999999999998]],
    [[0.0, -0.0, 0.0, 1.0, 0.0],
    [-0.08333333333333326, 0.5000000000000003, -1.5000000000000009, 0.833333333333334, 0.24999999999999983],
    [-0.08333333333333331, 0.33333333333333354, 0.49999999999999967, -1.6666666666666663, 0.9166666666666665],
    [0.49999999999999956, -3.0000000000000013, 6.000000000000003, -5.000000000000002, 1.5000000000000004],
    [1.0, -4.000000000000002, 6.000000000000003, -4.000000000000002, 1.0000000000000004]],
    [[0.0, -0.0, 1.0, -0.0, 0.0],
    [0.08333333333333337, -0.6666666666666667, 1.6653345369377348e-16, 0.6666666666666665, -0.08333333333333331],
    [-0.08333333333333348, 1.3333333333333335, -2.5, 1.3333333333333335, -0.08333333333333348],
    [-0.5, 1.0000000000000002, -3.885780586188048e-16, -0.9999999999999998, 0.49999999999999994],
    [1.0, -4.000000000000001, 6.000000000000002, -4.000000000000001, 1.0]],
    [[0.0, 1.0, 0.0, -0.0, 0.0],
    [-0.25000000000000094, -0.8333333333333302, 1.499999999999996, -0.49999999999999795, 0.08333333333333304],
    [0.9166666666666665, -1.666666666666666, 0.49999999999999944, 0.3333333333333335, -0.08333333333333331],
    [-1.4999999999999956, 4.999999999999986, -5.999999999999982, 2.9999999999999902, -0.499999999999998],
    [0.9999999999999956, -3.999999999999985, 5.9999999999999805, -3.9999999999999893, 0.999999999999998]],
    [[1.0, 0.0, -0.0, -0.0, 0.0],
    [-2.083333333333332, 3.9999999999999947, -2.9999999999999925, 1.3333333333333286, -0.2499999999999989],
    [2.9166666666666625, -8.66666666666665, 9.499999999999977, -4.666666666666652, 0.916666666666663],
    [-2.499999999999994, 8.99999999999998, -11.999999999999973, 6.999999999999983, -1.4999999999999958],
    [0.9999999999999982, -3.999999999999992, 5.9999999999999885, -3.999999999999993, 0.9999999999999984]]];

fn npsd(y: &Array1<f64>, d: usize, nn: usize) -> Array1<f64> {
    assert!(d < nn && d > 0 && nn < 10 && nn % 2 == 1);
    let nnn = nn / 2;
    let y0: Array1<f64> = y.slice(s![0..nn]).to_owned();
    let y1: Array1<f64> = y.slice(s![y.len()-nn..y.len()]).to_owned();
    let u: Array1<f64> = NPSC5[nnn][d].to_vec().into_iter().collect();
    let mut z: Array1<f64> = Array::zeros(y.len());
    for j in 0..y.len()-nn+1 {
        z[j+nnn] = u.dot(&y.slice(s![j..j+nn]))
    }
    for j in 0..nnn {
        z[j] = y0.dot(&NPSC5[nn-j-1][d].to_vec().into_iter().collect() as &Array1<f64>);
        z[y.len()-j-1] = y1.dot(&NPSC5[j][d].to_vec().into_iter().collect() as &Array1<f64>);
    }
    z
}

fn ppf(p: f64, dof: f64) -> f64 {
    let chi = ChiSquared::new(dof).unwrap();
    chi.inverse_cdf(p)
}
fn weighted_sum(value: &Array1<f64>, weight: &Array1<f64>) -> f64 {
    (value * weight).sum() / weight.sum()
}
pub fn is_conserved(mean: &Array1<f64>, std: &Array1<f64>, alpha: Option<f64>) -> bool {
    let n = mean.len();
    assert_eq!(n, std.len());
    {
        let tmp = mean[0];
        if mean.iter().all(|&a| a == tmp) {
            return true
        }
        if std.iter().any(|&a| a == 0.) {
            return false
        }
    }
    let alpha = alpha.unwrap_or(0.05);
    let dof = n as f64 - 1.;
    let weight: Array1<f64> = std.mapv(|x| 1. / x.powi(2));
    let mean_weighted = weighted_sum(mean, &weight);
    let chi_square_statistic = ((mean - mean_weighted).mapv(|x| x.powi(2)) * weight).sum();
    let critical_value = ppf(1.0 - alpha, dof);
    chi_square_statistic < critical_value
}

fn is_zero(mean: &Array1<f64>, std: &Array1<f64>, alpha: Option<f64>) -> bool {
    let n = mean.len();
    assert_eq!(n, std.len());
    {
        if mean.iter().all(|&a| a == 0.) {
            return true
        }
        if std.iter().any(|&a| a == 0.) {
            return false
        }
    }
    let alpha = alpha.unwrap_or(0.05);
    let dof = n as f64;
    let weight: Array1<f64> = std.mapv(|x| 1. / x.powi(2));
    let chi_square_statistic = (mean.mapv(|x| x.powi(2)) * weight).sum();
    let critical_value = ppf(1.0 - alpha, dof);
    chi_square_statistic < critical_value
}

fn weighted_mean_error(value: &Array1<f64>, std: &Array1<f64>) -> (f64, f64) {
    let n = value.len();
    assert_eq!(n, std.len());
    let weight = std.mapv(|x| 1. / x.powi(2));
    let mean = weighted_sum(value, &weight);
    let var = weighted_sum(&(value - mean).mapv(|x| x.powi(2)), &weight);
    (mean, (var / n as f64).sqrt())
}