use std::env;
use rosettamirror;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];
    match rosettamirror::run(dir) {
        Err(s) => println! ("{:?}", s),
        Ok(_) => (),
    }
}
