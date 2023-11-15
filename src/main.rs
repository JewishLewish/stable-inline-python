use pyo3::{prelude::*, types::PyDict};
use std::io::{self, Write};


fn main() {
    py_run! {
        print("hello world")
    };
}