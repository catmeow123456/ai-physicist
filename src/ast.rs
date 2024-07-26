use std::fmt;
use pyo3::prelude::*;
use pyo3::callback::IntoPyCallbackOutput;

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum Func {
    Sum,
    Prod,
    Forall,
}

#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Pow
}
impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Pow => write!(f, "**"),
        }
    }
}


#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
    Diff,
}
impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Diff => write!(f, "D"),
        }
    }
}

impl FromPyObject<'_> for Box<Exp> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let x = ob.extract::<Exp>()?;
        // println!("Extracted: {}", x);
        Ok(Box::new(x))
    }
}

impl IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Box<Exp>
{
    #[inline]
    fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
        Ok(self.into_py(py).as_ptr())
    }
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum Exp {
    // ExpConfig -> MeasureData -> ExpData
    Number {num: i32},
    Variable {name: String},
    VariableId {name: String, id: i32},
    UnaryExp {op: UnaryOp, exp: Box<Exp>},
    BinaryExp {left: Box<Exp>, op: BinaryOp, right: Box<Exp>},
    DiffExp {left: Box<Exp>, right: Box<Exp>, ord: i32},
}

#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub enum SExp {
    // MeasureData -> ExpData
    Mk {name: String, exp: Box<Exp>},
}

pub enum Expression {
    Exp {exp: Box<Exp>},
    SExp {sexp: Box<SExp>},
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number {num} => write!(f, "{}", num),
            Exp::Variable {name} => write!(f, "{}", name),
            Exp::VariableId {name, id} => write!(f, "{}[{}]", name, id),
            Exp::UnaryExp {op, exp} =>
                match op {
                    UnaryOp::Neg => write!(f, "-{}", exp),
                    UnaryOp::Diff => write!(f, "D.{}", exp),
                }
            Exp::BinaryExp {left, op, right} => write!(f, "({} {} {})", left, op, right),
            Exp::DiffExp {left, right, ord} => 
                match ord {
                    1 => write!(f, "D[{}]/D[{}]", left, right),
                    _ => write!(f, "D^{}[{}]/D[{}]^{}", ord, left, right,ord),
                }
        }
    }
}
impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Mk {name, exp} => write!(f, "{} |- {}", name, exp),
        }
    }
}