#[macro_use]
extern crate structopt;

mod cli;

use cli::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
