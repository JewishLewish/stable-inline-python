use std::default;

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

macro_rules! python_string {
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

impl py_context {
    fn new() -> py_context {
        py_context { ..Default::default() }
    }

    fn run(&self, input: &'static str) -> PyResult<()> {
        execute_python(Some(&self.variables), input)
    }
}



pub fn execute_python(py_vars: Option<&py_var>, input: &'static str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let locals: &PyDict = _define(py_vars, py);
        
        // Read Python code from user input

        // Execute the entered Python code
        let _ = py.run(&input, None, Some(locals)).unwrap();

        //let ret = locals.get_item("x").unwrap().unwrap();
        //print!("{:?}",ret);
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
