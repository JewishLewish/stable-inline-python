use pyo3::{prelude::*, types::PyDict};

#[macro_export]
macro_rules! py_run {
    ($code:expr) => {
        use crate::lib::execute_python;
        let code_str = stringify!($code);
        let _ = execute_python(code_str);
    };
}

pub fn execute_python(input: &'static str) -> PyResult<()> {
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

