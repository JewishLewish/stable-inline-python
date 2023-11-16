use stable_inline_rs::PyContext;

fn main() {
    
    let c = PyContext::new();

    c.run("x = 2"); //outputs 2
    let x = c.get::<String>("x");
    println!("{}",x.unwrap());

    c.run("del x");
    let x = c.get::<String>("x");
    println!("{}",x.unwrap()); //error!

    c.run("x = 2");
    let x = c.get::<u8>("x");
    println!("{}",x.unwrap()); //error!
}