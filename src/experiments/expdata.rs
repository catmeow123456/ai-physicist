// a data structure that represent n * m arrays
// where n is the number of times of experiments and m is the number of data points

use std::default::Default;
use std::f64::NAN;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use std::collections::HashSet;
use ndarray::{Array2};

// fn f_list<T, const N: usize> (a :&[T;N], b: &[T;N], f: fn(&T,&T) -> T) -> [T;N]
// where T: Copy, T: Default,
// {
//     let mut res = [Default::default(); N];
//     for i in 0..N {
//         res[i] = f(&a[i], &b[i]);
//     }
//     res
// }

#[derive(Debug, Clone)]
pub struct ExpData {
    pub n: usize,
    pub repeat_time: usize,
    pub data: Array2::<f64>,
    pub badpts: HashSet<usize>
}

impl ExpData {
    // Create a new ExpData structure
    pub fn new(data: Array2::<f64>) -> Self {
        let mut badpts = HashSet::new();
        let n = data.ncols();
        let repeat_time = data.nrows();
        for x in 0..n {
            for y in 0..repeat_time {
                if data[[y,x]].is_nan() || data[[y,x]].is_infinite(){
                    badpts.insert(x);
                    break;
                }
            }
        }
        let mut data: Array2::<f64> = data;
        for pt in badpts.iter() {
            for y in 0..repeat_time {
                data[[y,*pt]] = NAN;
            }
        }
        Self {n, repeat_time, data, badpts}
    }

    pub fn zero(n: usize, repeat_size: usize) -> ExpData {
        Self::new(Array2::zeros((repeat_size, n)))
    }

    pub fn gen_domain(&self) -> Vec<(usize,usize)> {
        let mut res = Vec::new();
        let mut last = 0;
        for i in self.badpts.iter() {
            if *i > last {
                res.push((last, *i - 1))
            }
            last = *i + 1;
        }
        res
    }
}


// define the Display trait for ExpData
impl fmt::Display for ExpData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExpData({}, {}) =\n", self.repeat_time, self.n).unwrap();
        // print the n*m array
        for j in 0..self.repeat_time {
            for i in 0..self.n {
                if self.badpts.contains(&i) {
                    write!(f, "_ ").unwrap();
                    continue;
                }
                write!(f, "{} ", self.data[[j,i]]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

//
// Operator overloading
//


// implement the Add trait for ExpData
impl Add for ExpData{
    type Output = ExpData;
    fn add(self, other: ExpData) -> ExpData {
        ExpData::new(&self.data + &other.data)
    }
}
impl Add for &ExpData {
    type Output = ExpData;
    fn add(self, other: &ExpData) -> ExpData {
        ExpData::new(&self.data + &other.data)
    }
}

// implement the Sub trait for ExpData
impl Sub for ExpData {
    type Output = ExpData;
    fn sub(self, other: ExpData) -> ExpData {
        ExpData::new(&self.data - &other.data)
    }
}
impl Sub for &ExpData {
    type Output = ExpData;
    fn sub(self, other: &ExpData) -> ExpData {
        ExpData::new(&self.data - &other.data)
    }
}

// implement the Mul trait for ExpData
impl Mul for ExpData {
    type Output = ExpData;
    fn mul(self, other: ExpData) -> ExpData {
        ExpData::new(&self.data * &other.data)
    }
}
impl Mul for &ExpData {
    type Output = ExpData;
    fn mul(self, other: &ExpData) -> ExpData {
        ExpData::new(&self.data * &other.data)
    }
}

// implement the Div trait for ExpData
impl Div for ExpData {
    type Output = ExpData;
    fn div(self, other: ExpData) -> ExpData {
        ExpData::new(&self.data / &other.data)
    }
}
impl Div for &ExpData {
    type Output = ExpData;
    fn div(self, other: &ExpData) -> ExpData {
        ExpData::new(&self.data / &other.data)
    }
}

// implement the AddAssign trait for ExpData
impl AddAssign for ExpData {
    fn add_assign(&mut self, other: ExpData) {
        self.data += &other.data;
    }
}