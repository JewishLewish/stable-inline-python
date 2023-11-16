use std::str::FromStr;

use pyo3::{prelude::*, types::PyDict};


pub struct PyContext {
    pub variables: PyVar
}

impl Default for PyContext {
    fn default() -> Self {
        PyContext { variables: Default::default() }
    }
}


pub struct PyVar {
    pub locals: Py<PyDict>
}

impl Default for PyVar {
    fn default() -> Self {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Self { locals: PyDict::new(py).into() }
        })
    }
}

impl PyContext {
    pub fn new() -> PyContext {
        PyContext { ..Default::default() }
    }

    pub fn run(&self, input: &'static str) {
        let _ = self.execute_python(Some(&self.variables), input);
    }
     
    pub fn get<T: FromStr>(&self, input: &'static str) -> Result<T, <T as FromStr>::Err> {

        pyo3::prepare_freethreaded_python();

        let out = Python::with_gil(|py| {
            let locals: &PyDict = self._define(Some(&self.variables), py);
            
            let ret = locals.get_item(input).unwrap().unwrap();
            format!("{}",ret)
        });
        out.parse::<T>()
    }

    fn execute_python(&self, py_vars: Option<&PyVar>, input: &'static str) -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
    
        Python::with_gil(|py| {
            let locals: &PyDict = self._define(py_vars, py);
            
            let _ = py.run(&input, None, Some(locals)).unwrap();
        });
    
        Ok(())
    }

    fn _define<'a>(&self, py_vars: Option<&'a PyVar>, py: Python<'a>) -> &'a PyDict {
        if py_vars.is_none() {
            PyDict::new(py).into()
        } else {
            py_vars.unwrap().locals.as_ref(py)
        }
    }
    
}
