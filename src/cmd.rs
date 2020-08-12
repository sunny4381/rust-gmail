pub(crate) mod base;
pub(crate) mod init;
pub(crate) mod send;
pub(crate) mod refresh;
pub(crate) mod whoami;

use clap::Clap;

use crate::error::Error;
use crate::cmd::base::Cmd;
use crate::cmd::init::InitCmd;
use crate::cmd::send::SendCmd;
use crate::cmd::refresh::RefreshCmd;
use crate::cmd::whoami::WhoamiCmd;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "NAKANO Hideo <pinarello.marvel@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Init(InitCmd),
    Send(SendCmd),
    Refresh(RefreshCmd),
    Whoami(WhoamiCmd),
}

impl Cmd for Opts {
    fn run(&self) -> Result<(), Error> {
        match self.subcmd {
            SubCommand::Init(ref cmd) => cmd.run(),
            SubCommand::Send(ref cmd) => cmd.run(),
            SubCommand::Refresh(ref cmd) => cmd.run(),
            SubCommand::Whoami(ref cmd) => cmd.run(),
        }
    }
}
