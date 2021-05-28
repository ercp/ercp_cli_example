#![deny(unsafe_code)]

use ercp_cli_example::{Cli, Opts};
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();
    let mut cli = Cli::new(opts);
    cli.run()
}
