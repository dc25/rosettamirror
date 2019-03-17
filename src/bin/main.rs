use std::env;
use rosettamirror;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];
    if let Err(s) = rosettamirror::run(dir) {
        println! ("{:?}", s);
    }
}
