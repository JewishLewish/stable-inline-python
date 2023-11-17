# 0.1.0
* PyO3 Wrapper Stable 

# 0.1.1
* c.run returns ``-> Result<(), pyo3::PyErr>``

# 0.1.2
* added ``py_eval!`` macro
```rust
fn main() {
    py_eval!{r#"print("Hello world")"#}; //outputs Hello world, doesn't save variables
}```