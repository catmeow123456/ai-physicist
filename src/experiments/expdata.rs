// A Datastruct which represents different kinds of status of an experiment data (rely on "t")

use super::{
    normaldata::NormalData,
    constdata::ConstData,
};
use pyo3::prelude::*;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};
use ndarray::Array2;



#[pyclass]
#[derive(Debug, Clone)]
pub enum ExpData {
    Normal { content: NormalData },
    Const { content: ConstData },
    Zero {},
    Err {},
}

#[pymethods]
impl ExpData {
    #[getter]
    pub fn is_const(&self) -> bool {
        match self {
            ExpData::Const { content: _ } => true,
            _ => false
        }
    }
    #[getter]
    pub fn is_conserved(&self) -> bool {
        match self {
            ExpData::Const { content: _ } => true,
            ExpData::Zero { } => true,
            _ => false
        }
    }
    #[getter]
    pub fn is_zero(&self) -> bool {
        match self {
            ExpData::Zero { } => true,
            _ => false
        }
    }
    #[getter]
    pub fn is_err(&self) -> bool {
        match self {
            ExpData::Err { } => true,
            _ => false
        }
    }
    #[getter]
    fn is_normal(&self) -> bool {
        match self {
            ExpData::Normal { content: _ } => true,
            _ => false
        }
    }
    #[getter]
    fn n(&self) -> usize {
        match self {
            ExpData::Normal { content } => content.n,
            ExpData::Const { content: _ } => panic!("ConstData has no n"),
            ExpData::Zero { } => panic!("ZeroData has no n"),
            ExpData::Err { } => panic!("ErrData has no n"), 
        }
    }
    #[getter]
    fn normal_data(&self) -> NormalData {
        match self {
            ExpData::Normal { content } => content.clone(),
            _ => panic!("unwrap_normal_data called on non-NormalData")
        }
    }
    #[getter]
    fn const_data(&self) -> ConstData {
        match self {
            ExpData::Const { content } => content.clone(),
            _ => panic!("unwrap_const_data called on non-ConstData")
        }
    }
    #[staticmethod]
    fn from_elem(mean: f64, std: f64, n: usize, repeat_time: usize) -> ExpData {
        ExpData::Normal { content: NormalData::from_elem(mean, std, n, repeat_time) }
    }
    #[getter]
    fn repeat_time(&self) -> usize {
        match self {
            ExpData::Normal { content } => content.repeat_time,
            ExpData::Const { content: _ } => panic!("ConstData has no repeat_time"),
            ExpData::Zero { } => panic!("ZeroData has no repeat_time"),
            ExpData::Err { } => panic!("ErrData has no repeat_time"),
        }
    }
    fn __add__(&self, other: &ExpData) -> PyResult<ExpData> {
        Ok(self + other)
    }
    fn __sub__(&self, other: &ExpData) -> PyResult<ExpData> {
        Ok(self - other)
    }
    fn __mul__(&self, other: &ExpData) -> PyResult<ExpData> {
        Ok(self * other)
    }
    fn __truediv__(&self, other: &ExpData) -> PyResult<ExpData> {
        Ok(self / other)
    }
    fn __neg__(&self) -> PyResult<ExpData> {
        Ok(-self.clone())
    }
    fn __powi__(&self, other: i32) -> PyResult<ExpData> {
        Ok(self.powi(other))
    }
    fn __diff__(&self, other: &ExpData) -> PyResult<ExpData> {
        Ok(self.diff(other))
    }
    fn __difftau__(&self) -> PyResult<ExpData> {
        Ok(self.diff_tau())
    }
    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

impl ExpData {
    #[inline]
    pub fn unwrap_normal_data(&self) -> &NormalData {
        match self {
            ExpData::Normal { content } => content,
            _ => panic!("unwrap_normal_data called on non-NormalData")
        }
    }
    #[inline]
    pub fn unwrap_const_data(&self) -> &ConstData {
        match self {
            ExpData::Const { content } => content,
            _ => panic!("unwrap_const_data called on non-ConstData")
        }
    }
    #[inline]
    pub fn from_arr2(arr: Array2<f64>) -> ExpData {
        ExpData::from_normal_data(NormalData::new(arr))
    }
    #[inline]
    pub fn from_const(mean: f64, std: f64) -> ExpData {
        ExpData::from_const_data(ConstData::new(mean, std))
    }
    #[inline]
    pub fn from_exact_const(value: i32) -> ExpData {
        ExpData::from_const_data(ConstData::Exact{ value })
    }
    #[inline]
    fn from_normal_data(content: NormalData) -> ExpData {
        if content.badpts.len() > content.n / 4 {
            ExpData::Err { }
        }
        else
        if content.is_zero() {
            ExpData::Zero { }
        }
        else
        if content.is_conserved() {
            ExpData::Const { content: content.to_const_data() }
        }
        else {
            ExpData::Normal { content }
        }
    }
    #[inline]
    fn from_const_data(content: ConstData) -> ExpData {
        match content {
            ConstData::Data { mean, std } => {
                if std > mean.abs() * 1000.0 {
                    // 等于 0 的置信度非常高
                    ExpData::Zero { }
                }
                else
                if std > mean.abs() / 5.0 {
                    // 处于 0 和 const 之间的模糊地带的数据，不予考虑
                    ExpData::Err { }
                }
                else {
                    ExpData::Const { content }
                }
            },
            ConstData::Exact { value } => {
                if value == 0 {
                    ExpData::Zero { }
                }
                else {
                    ExpData::Const { content }
                }
            }
        }
    }
    #[inline]
    fn to_normal_data(&self, n: usize, repeat_time: usize) -> NormalData {
        match self {
            ExpData::Normal { content } => {
                assert_eq!(n, content.n);
                assert_eq!(repeat_time, content.repeat_time);
                content.clone()
            },
            ExpData::Const { content } => {
                match content {
                    ConstData::Data { mean, std } => NormalData::from_elem(*mean, *std, n, repeat_time),
                    ConstData::Exact { value } => NormalData::from_elem(*value as f64, 0.0, n, repeat_time),
                }
            },
            ExpData::Zero { } => NormalData::zero(n, repeat_time),
            ExpData::Err { } => panic!("Cannot convert ErrData to NormalData"),
        }
    }
}


impl fmt::Display for ExpData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpData::Normal { content } => write!(f, "{}", content),
            ExpData::Const { content } => write!(f, "{}", content),
            ExpData::Zero { } => write!(f, "ExpData.Zero"),
            ExpData::Err { } => write!(f, "ExpData.Err"),
        }
    }
}

//
// Operator overloading
//

pub trait Diff<Rhs = Self> {
    type Output;
    fn diff(&self, other: Rhs) -> Self::Output;
    fn diff_n(&self, other: Rhs, n: usize) -> Self::Output;
}

// implement the Add trait for ExpData
impl Add for ExpData{
    type Output = ExpData;
    #[inline]
    fn add(self, other: ExpData) -> ExpData {
        (&self) + (&other)
    }
}
impl Add for &ExpData {
    type Output = ExpData;
    #[inline]
    fn add(self, other: &ExpData) -> ExpData {
        if self.is_err() || other.is_err() {
            return ExpData::Err { }
        }
        if self.is_zero() { return other.clone(); }
        if other.is_zero() { return self.clone(); }
        if self.is_const() && other.is_const() {
            ExpData::from_const_data(self.unwrap_const_data() + other.unwrap_const_data())
        } else {
            let n = if !self.is_const() { self.n() } else { other.n() };
            let repeat_time = if !self.is_const() { self.repeat_time() } else { other.repeat_time() };
            ExpData::from_normal_data(self.to_normal_data(n, repeat_time) + other.to_normal_data(n, repeat_time))
        }
    }
}

// implement the Sub trait for ExpData
impl Sub for ExpData {
    type Output = ExpData;
    #[inline]
    fn sub(self, other: ExpData) -> ExpData {
        (&self) - (&other)
    }
}
impl Sub for &ExpData {
    type Output = ExpData;
    #[inline]
    fn sub(self, other: &ExpData) -> ExpData {
        if self.is_err() || other.is_err() {
            return ExpData::Err { }
        }
        if self.is_zero() { return -other; }
        if other.is_zero() { return self.clone(); }
        if self.is_const() && other.is_const() {
            ExpData::from_const_data(self.unwrap_const_data() - other.unwrap_const_data())
        } else {
            let n = if !self.is_const() { self.n() } else { other.n() };
            let repeat_time = if !self.is_const() { self.repeat_time() } else { other.repeat_time() };
            ExpData::from_normal_data(self.to_normal_data(n, repeat_time) - other.to_normal_data(n, repeat_time))
        }
    }
}

// implement the Mul trait for ExpData
impl Mul for ExpData {
    type Output = ExpData;
    #[inline]
    fn mul(self, other: ExpData) -> ExpData {
        (&self) * (&other)
    }
}
impl Mul for &ExpData {
    type Output = ExpData;
    #[inline]
    fn mul(self, other: &ExpData) -> ExpData {
        if self.is_err() || other.is_err() {
            return ExpData::Err { }
        }
        if self.is_zero() || other.is_zero() {
            ExpData::Zero { }
        } else if self.is_const() && other.is_const() {
            ExpData::from_const_data(self.unwrap_const_data() * other.unwrap_const_data())
        } else {
            let n = if !self.is_const() { self.n() } else { other.n() };
            let repeat_time = if !self.is_const() { self.repeat_time() } else { other.repeat_time() };
            ExpData::from_normal_data(self.to_normal_data(n, repeat_time) * other.to_normal_data(n, repeat_time))
        }
    }
}

// implement the Div trait for ExpData
impl Div for ExpData {
    type Output = ExpData;
    #[inline]
    fn div(self, other: ExpData) -> ExpData {
        (&self) / (&other)
    }
}
impl Div for &ExpData {
    type Output = ExpData;
    #[inline]
    fn div(self, other: &ExpData) -> ExpData {
        if self.is_err() || other.is_err() || other.is_zero() {
            return ExpData::Err { }
        }
        if self.is_zero() {
            ExpData::Zero {  }
        }
        else if self.is_const() && other.is_const() {
            ExpData::from_const_data(self.unwrap_const_data() / other.unwrap_const_data())
        }
        else {
            let n = if !self.is_const() { self.n() } else { other.n() };
            let repeat_time = if !self.is_const() { self.repeat_time() } else { other.repeat_time() };
            ExpData::from_normal_data(self.to_normal_data(n, repeat_time) / other.to_normal_data(n, repeat_time))
        }
    }
}

// implement the AddAssign trait for ExpData
impl AddAssign for ExpData {
    #[inline]
    fn add_assign(&mut self, other: ExpData) {
        *self = &*self + &other;
    }
}

impl Neg for ExpData {
    type Output = ExpData;
    #[inline]
    fn neg(self) -> ExpData {
        -&self
    }
}

impl Neg for &ExpData {
    type Output = ExpData;
    #[inline]
    fn neg(self) -> ExpData {
        match self {
            ExpData::Normal { content } => ExpData::from_normal_data(-content),
            ExpData::Const { content } => ExpData::from_const_data(-content),
            ExpData::Zero { } => ExpData::Zero { },
            ExpData::Err { } => ExpData::Err { }
        }
    }
}

impl ExpData {
    #[inline]
    pub fn powi(&self, other: i32) -> ExpData {
        match self {
            ExpData::Normal { content } => ExpData::from_normal_data(content.powi(other)),
            ExpData::Const { content } => ExpData::from_const_data(content.powi(other)),
            ExpData::Zero { } => ExpData::Zero { },
            ExpData::Err { } => ExpData::Err { }
        }
    }
    #[inline]
    pub fn diff_tau(&self) -> ExpData {
        match self {
            ExpData::Normal { content } => ExpData::from_normal_data(content.diff_tau()),
            ExpData::Const { content: _ } => ExpData::Zero { },
            ExpData::Zero { } => ExpData::Zero { },
            ExpData::Err { } => ExpData::Err { }
        }
    }
}

impl Diff for &ExpData {
    type Output = ExpData;
    #[inline]
    fn diff(&self, other: &ExpData) -> ExpData {
        if self.is_zero() || self.is_const() {
            ExpData::Zero { }
        }
        else if other.is_zero() || other.is_const() {
            ExpData::Err { }
        }
        else {
            if other.unwrap_normal_data().is_conserved_piecewise() {
                ExpData::Err { }
            }
            else if self.unwrap_normal_data().is_conserved_piecewise() {
                ExpData::Zero { }
            }
            else {
                ExpData::from_normal_data(self.unwrap_normal_data().diff_tau() / other.unwrap_normal_data().diff_tau())
            }
        }
    }
    #[inline]
    fn diff_n(&self, other: &ExpData, n: usize) -> ExpData {
        assert!(n > 0 && n < 5);
        if n == 1 {
            self.diff(other)
        } else {
            (&self.diff(other)).diff_n(other, n-1)
        }
    }
}

