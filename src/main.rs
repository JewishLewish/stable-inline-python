mod lib;

use crate::lib::{execute_python, py_vars};

#[macro_export]
macro_rules! py_run {
    ($py_vars:expr, $($code:tt)*) => {
        {
            let code_str = stringify!($($code)*);
            let _ = execute_python($py_vars, code_str);
        }
    };
}

fn main() {
    let py_vars = py_vars { ..Default::default() };

    py_run! {
        &py_vars,
        x = 2
    };

    py_run! {
        &py_vars,
        print(x)
    };
}