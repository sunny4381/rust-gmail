pub(crate) mod base;
pub(crate) mod init;
pub(crate) mod send;
pub(crate) mod refresh;
pub(crate) mod whoami;

use clap::Clap;

use crate::error::Error;
use crate::cmd::base::{Cmd, retry_cmd};
use crate::cmd::init::InitCmd;
use crate::cmd::send::SendCmd;
use crate::cmd::refresh::RefreshCmd;
use crate::cmd::whoami::WhoamiCmd;

const MAX_TRIES: u16 = 3;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "NAKANO Hideo <pinarello.marvel@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Init(InitCmd),
    Refresh(RefreshCmd),
    Whoami(WhoamiCmd),
    Send(SendCmd),
}

impl Cmd for Opts {
    fn run(&self) -> Result<(), Error> {
        match self.subcmd {
            SubCommand::Init(ref cmd) => cmd.run(),
            SubCommand::Refresh(ref cmd) => cmd.run(),
            SubCommand::Whoami(ref cmd) => retry_cmd(MAX_TRIES, cmd, &RefreshCmd {}),
            SubCommand::Send(ref cmd) => retry_cmd(MAX_TRIES, cmd, &RefreshCmd {}),
        }
    }
}
