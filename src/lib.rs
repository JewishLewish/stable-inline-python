use std::default;

use pyo3::{prelude::*, types::PyDict};



pub struct py_context {
    pub locals: Py<PyDict>
}

impl Default for py_context {
    fn default() -> Self {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Self { locals: PyDict::new(py).into() }
        })
    }
}



pub fn execute_python(py_vars: Option<&py_context>, input: &'static str) -> PyResult<()> {
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

fn _define<'a>(py_vars: Option<&'a py_context>, py: Python<'a>) -> &'a PyDict {
    if py_vars.is_none() {
        PyDict::new(py).into()
    } else {
        py_vars.unwrap().locals.as_ref(py)
    }
}
