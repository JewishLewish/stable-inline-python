use stable_inline_rs::PyContext;


fn main() {
    let c = PyContext::new();

    c.run("x=2");
    let x = c.get::<String>("x");
    print!("{}",x.unwrap());
}