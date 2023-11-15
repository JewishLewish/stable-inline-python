mod lib;

use crate::lib::*;



fn main() {
    let c = PyContext { ..Default::default() };

    c.run("x=2");
    let x = c.get::<i32>("x");
    print!("{}",x.unwrap());
}