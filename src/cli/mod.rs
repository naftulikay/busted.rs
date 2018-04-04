use std::path::PathBuf;

use structopt;

#[derive(Debug,StructOpt)]
#[structopt(raw(setting="structopt::clap::AppSettings::ColoredHelp"))]
/// A collection of utilities written in Rust.
pub struct Opt {
    /// Enable verbose application logging.
    #[structopt(short="v", long="verbose")]
    pub verbose: bool,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug,StructOpt)]
pub enum Command {
    /// Render a set of Jinja/Askama templates.
    #[structopt(name="render", raw(setting="structopt::clap::AppSettings::ColoredHelp"))]
    Render {
        /// A list of Jinja/Askama template files to render in order.
        #[structopt(parse(from_os_str))]
        templates: Vec<PathBuf>,
        #[structopt(short="o", long="output-file", parse(from_os_str))]
        output_file: Option<PathBuf>,
    },
}
