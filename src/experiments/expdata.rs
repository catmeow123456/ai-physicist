// a data structure that represent n * m arrays
// where n is the number of times of experiments and m is the number of data points

use std::f64::NAN;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};
use std::collections::HashSet;
use ndarray::{s, Array, Array1, Array2, Array3};
use ndarray_linalg::Solve;

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

    pub fn from_elem(value: f64, n: usize, repeat_size: usize) -> ExpData {
        Self::new(Array2::from_elem((repeat_size, n), value))
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

    pub fn plot_over_t(&self, name: &str, t: &ExpData) {
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

impl Neg for ExpData {
    type Output = ExpData;
    fn neg(self) -> ExpData {
        ExpData::new(-&self.data)
    }
}

impl ExpData {
    pub fn pow(&self, other: &ExpData) -> ExpData {
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
        ExpData::new(res)
    }
    pub fn diff_tau(&self) -> ExpData {
        npsd(self, 1, 5)
    }
    pub fn diff(&self, other: &ExpData) -> ExpData {
        self.diff_tau() / other.diff_tau()
    }
    pub fn diff_n(&self, other: &ExpData, n: usize) -> ExpData {
        assert!(n > 0 && n < 5);
        if n == 1 {
            self.diff(other)
        } else {
            self.diff(other).diff_n(other, n-1)
        }
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
        let mut a: Array2<f64> = Array2::<f64>::zeros((n*2-1, n));
        for i in 0..n {
            a.column_mut(i).assign(&col.mapv(|x| x.powf(i as f64) / c[i]));
        }
        let b: Array2<f64> = Array::eye(n);
        let r: Array1<f64> = col.mapv(|x| x.powf(n as f64) / c[n]);
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

fn npsd(y: &ExpData, d: usize, nn: usize) -> ExpData {
    assert!(d < nn && d > 0 && nn < 10 && nn % 2 == 1);
    let c: Array2<f64> = NPSCoefficient::new(nn).c.slice(s![.., d, ..]).to_owned();
    let nnn = nn / 2;
    let y0: Array2<f64> = y.data.slice(s![.., 0..nn]).to_owned();
    let y1: Array2<f64> = y.data.slice(s![.., y.n-nn..y.n]).to_owned();
    let u: Array1<f64> = c.row(nnn).to_owned();
    let mut z: Array2<f64> = Array::zeros((y.repeat_time, y.n));
    for i in 0..y.repeat_time {
        for j in 0..y.n-nn+1 {
            z[[i, j+nnn]] = u.dot(&y.data.slice(s![i, j..j+nn]))
        }
        for j in 0..nnn {
            z[[i, j]] = y0.row(i).dot(&c.row(nn-j-1));
            z[[i, y.n-j-1]] = y1.row(i).dot(&c.row(j));
        }
    }
    ExpData::new(z)
}
