use rosettamirror;
use std::error::Error;
use std::env;
use std::fs;
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
    fs::DirBuilder::new().recursive(true).create(&opt.directory)?;
	env::set_current_dir(&opt.directory)?;
    rosettamirror::run(opt.all)
}
