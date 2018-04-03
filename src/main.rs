#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug,StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// A collection of utilities written in Rust.
struct Opt {
    /// Enable verbose application logging.
    #[structopt(short="v", long="verbose")]
    verbose: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug,StructOpt)]
enum Command {
    /// Render a Jinja/Askama template.
    #[structopt(name = "render")]
    Render {
        acorns: u32
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
