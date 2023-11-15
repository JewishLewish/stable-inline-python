# stable-inline-rs
Stable Ver. of Inline Rust

# QuickStart
```rust
fn main() {
    py_run! {
        print("hello world")
    };
}
```

```rust
fn main() {
    let py_vars = py_context { ..Default::default() };

    python_string! {Some(&py_vars),
        r#"
def hi():
    print("Hello world!")

hi()
        "#
    };
}```

```rust
use stable_inline_rs::PyContext;


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