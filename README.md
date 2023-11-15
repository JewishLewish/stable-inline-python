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