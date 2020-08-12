use clap::Clap;

use super::base::{Cmd, retry};
use crate::config::Config;
use crate::error::Error;
use crate::goauth;
use crate::cmd::refresh::RefreshCmd;

#[derive(Clap)]
pub struct WhoamiCmd {
}

impl Cmd for WhoamiCmd {
    fn run(&self) -> Result<(), Error> {
        let run1 = || -> Result<(), Error> {
            let config = Config::load("default")?;
            let user_info = goauth::user_info(&config.access_token)?;

            println!("{}", user_info.email);

            Ok(())
        };

        let before_retry = || -> Result<(), Error> {
            RefreshCmd {}.run()
        };

        retry(3, run1, before_retry)
    }
}
