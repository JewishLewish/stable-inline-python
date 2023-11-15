mod lib;

use crate::lib::*;

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

fn main() {
    let py_vars = py_context { ..Default::default() };

    python_string! {Some(&py_vars),
        "x = 2
print(x)"
    };

    python_string!{Some(&py_vars),
        "print(x)"
    }
}