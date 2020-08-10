mod cmd;
mod config;
mod error;
mod goauth;

use std::io::{self, Write};
use clap::Clap;

use crate::cmd::base::Cmd;
use crate::cmd::Opts;
use crate::error::Error;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.run() {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
