mod lib;

use crate::lib::*;



fn main() {
    let c = py_context { ..Default::default() };
    let py_vars = py_var { ..Default::default() };

    python_string! {Some(&py_vars),
        r#"
def hi():
    print("Hello world!")

hi()
        "#
    };
}