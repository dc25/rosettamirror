use rosettamirror;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];
    if let Err(s) = rosettamirror::run(dir) {
        println!("{:?}", s);
    }
}
