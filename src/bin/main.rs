extern crate reqwest;
extern crate url;
extern crate rustc_serialize;

use std::fs::*;
use std::io::prelude::*;

use rosettamirror;
 
fn main() {
    rosettamirror::run();
}
