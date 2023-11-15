use pyo3::{prelude::*, types::PyDict};
use std::io::{self, Write};

fn execute_python(input: &'static str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
            let locals = PyDict::new(py);
            // Read Python code from user input

            // Execute the entered Python code
            let _ = py.run(&input, None, Some(locals)).unwrap();

            //let ret = locals.get_item("x").unwrap().unwrap();
            //print!("{:?}",ret);
    });

    Ok(())
}


macro_rules! py_run {
    ($code:expr) => {
        let code_str = stringify!($code);
        let _ = execute_python(code_str);
    };
}