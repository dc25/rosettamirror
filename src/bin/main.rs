use rosettamirror;
use std::error::Error;
use structopt::StructOpt;

extern crate structopt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "d", long = "directory")]
    directory: String,

    #[structopt(short = "a", long = "all")]
    all: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    rosettamirror::run(&opt.directory, opt.all)
}
