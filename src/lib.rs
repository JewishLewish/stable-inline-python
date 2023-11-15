use std::default;

use pyo3::{prelude::*, types::PyDict};


pub struct py_vars {
    pub locals: Py<PyDict>
}

impl  Default for py_vars {
    fn default() -> Self {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Self { locals: PyDict::new(py).into() }
        })
    }
}



pub fn execute_python(py_vars: &py_vars, input: &'static str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
            let locals = &py_vars.locals.as_ref(py);
            // Read Python code from user input

            // Execute the entered Python code
            let _ = py.run(&input, None, Some(locals)).unwrap();

            //let ret = locals.get_item("x").unwrap().unwrap();
            //print!("{:?}",ret);
    });

    Ok(())
}

