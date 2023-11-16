use stable_inline_rs::PyContext;

fn main() {
    
    let c = PyContext::new();

    let _ = c.run_file("test.py");

    c.run("hi()");
}