use std::{default, str::FromStr};

use pyo3::{prelude::*, types::PyDict};


#[macro_export]
macro_rules! py_run {
    ($py_vars:expr, $($code:tt)*) => {
        {
            println!("{:?}",$($code)*);
            let code_str = stringify!($($code)*);
            let _ = execute_python($py_vars, code_str);
        }
    };
}

#[macro_export]
macro_rules! python {
    ($py_vars:expr, $code:expr) => {
        {
            let _ = execute_python($py_vars, $code);
        }
    };
}

pub struct py_context {
    pub variables: py_var
}

impl Default for py_context {
    fn default() -> Self {
        py_context { variables: Default::default() }
    }
}


pub struct py_var {
    pub locals: Py<PyDict>
}

impl Default for py_var {
    fn default() -> Self {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Self { locals: PyDict::new(py).into() }
        })
    }
}

pub struct SingleGen<T>(T);

impl py_context {
    pub fn new() -> py_context {
        py_context { ..Default::default() }
    }

    pub fn run(&self, input: &'static str) {
        let _ = execute_python(Some(&self.variables), input);
    }
     
    pub fn get<T: FromStr>(&self, input: &'static str) -> Result<T, <T as FromStr>::Err> {

        pyo3::prepare_freethreaded_python();

        let out = Python::with_gil(|py| {
            let locals: &PyDict = _define(Some(&self.variables), py);
            
            let ret = locals.get_item(input).unwrap().unwrap();
            format!("{}",ret)
        });
        out.parse::<T>()
    }

}



pub fn execute_python(py_vars: Option<&py_var>, input: &'static str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let locals: &PyDict = _define(py_vars, py);
        
        let _ = py.run(&input, None, Some(locals)).unwrap();
    });

    Ok(())
}

fn _define<'a>(py_vars: Option<&'a py_var>, py: Python<'a>) -> &'a PyDict {
    if py_vars.is_none() {
        PyDict::new(py).into()
    } else {
        py_vars.unwrap().locals.as_ref(py)
    }
}
