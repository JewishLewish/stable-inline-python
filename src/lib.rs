use std::{str::FromStr, fs};

use pyo3::{prelude::*, types::PyDict};

/// Represents a context for executing Python code.
pub struct PyContext {
    /// Stores Python variables and their values.
    pub variables: PyVar
}

impl Default for PyContext {
    /// Constructs a new `PyContext` with default values.
    fn default() -> Self {
        PyContext { variables: Default::default() }
    }
}

/// Represents Python variables and their values.
pub struct PyVar {
    /// Stores local Python variables.
    pub locals: Py<PyDict>
}

impl Default for PyVar {
    /// Constructs a new `PyVar` with default values.
    fn default() -> Self {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Self { locals: PyDict::new(py).into() }
        })
    }
}

impl PyContext {
    /// Constructs a new empty `PyContext`.
    pub fn new() -> PyContext {
        PyContext { ..Default::default() }
    }

    /// Executes Python code provided as input.
    ///
    /// # Arguments
    ///
    /// * `input` - A string containing the Python code to execute.
    pub fn run(&self, input: &str) {
        let _ = self.execute_python(Some(&self.variables), &input);
    }

    /// Executes Python code from a file specified by `file`.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the file containing Python code.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue reading the file or executing Python code.
    pub fn run_file(&self, file: &str) -> Result<(), PyErr> {
        let contents = fs::read_to_string(&file)?;
        self.execute_python(Some(&self.variables), &contents)
    }
     
    
    /// Retrieves the value of a Python variable identified by `input`.
    ///
    /// # Arguments
    ///
    /// * `input` - The name of the Python variable to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing of the variable value fails.
    pub fn get<T: FromStr>(&self, input: &str) -> Result<T, <T as FromStr>::Err> {

        pyo3::prepare_freethreaded_python();

        let out = Python::with_gil(|py| {
            let locals: &PyDict = self._define(Some(&self.variables), &py);
            
            let ret = locals.get_item(&input).unwrap().unwrap();
            format!("{}",ret)
        });
        out.parse::<T>()
    }

    fn execute_python(&self, py_vars: Option<&PyVar>, input: &str) -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
    
        Python::with_gil(|py| {
            let locals: &PyDict = self._define(py_vars, &py);
            
            let _ = py.run(&input, None, Some(locals)).unwrap();
        });
    
        Ok(())
    }

    /// Defines Python variables for execution.
    ///
    /// # Arguments
    ///
    /// * `py_vars` - Optional Python variables and their values.
    /// * `py` - A Python interpreter.
    ///
    /// # Returns
    ///
    /// A reference to a Python dictionary containing the defined variables.
    fn _define<'a>(&self, py_vars: Option<&'a PyVar>, py: &Python<'a>) -> &'a PyDict {
        if py_vars.is_none() {
            PyDict::new(*py).into()
        } else {
            py_vars.unwrap().locals.as_ref(*py)
        }
    }
    
}
