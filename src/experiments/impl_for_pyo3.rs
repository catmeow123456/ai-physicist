use crate::ast::{Exp, IExpConfig, SExp, ObjAttrExp, MeasureType};
use pyo3::prelude::*;
use pyo3::callback::IntoPyCallbackOutput;

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
impl FromPyObject<'_> for Box<IExpConfig> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let x = ob.extract::<IExpConfig>()?;
        // println!("Extracted: {}", x);
        Ok(Box::new(x))
    }
}
impl IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Box<IExpConfig>
{
    #[inline]
    fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
        Ok(self.into_py(py).as_ptr())
    }
}
impl FromPyObject<'_> for Box<SExp> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let x = ob.extract::<SExp>()?;
        // println!("Extracted: {}", x);
        Ok(Box::new(x))
    }
}
impl IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Box<SExp>
{
    #[inline]
    fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
        Ok(self.into_py(py).as_ptr())
    }
}

impl FromPyObject<'_> for Box<ObjAttrExp> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let x = ob.extract::<ObjAttrExp>()?;
        // println!("Extracted: {}", x);
        Ok(Box::new(x))
    }
}
impl IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Box<ObjAttrExp>
{
    #[inline]
    fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
        Ok(self.into_py(py).as_ptr())
    }
}

impl FromPyObject<'_> for Box<MeasureType> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let x = ob.extract::<MeasureType>()?;
        // println!("Extracted: {}", x);
        Ok(Box::new(x))
    }
}
impl IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Box<MeasureType>
{
    #[inline]
    fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
        Ok(self.into_py(py).as_ptr())
    }
}