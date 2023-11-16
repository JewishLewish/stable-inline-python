# stable-inline-python
Stable Ver. of Inline Rust

# QuickStart
```rust
use stable_inline_python::PyContext;


fn main() {
    let c = PyContext::new();

    c.run("x = 2"); //outputs 2
    let x = c.get::<String>("x");
    println!("{}",x.unwrap());

    c.run("del x");
    let x = c.get::<String>("x");
    println!("{}",x.unwrap()); //error!
}
```