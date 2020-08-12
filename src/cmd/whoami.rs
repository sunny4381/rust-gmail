use clap::Clap;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;
use crate::goauth;

#[derive(Clap)]
pub struct WhoamiCmd {
}

impl Cmd for WhoamiCmd {
    fn run(&self) -> Result<(), Error> {
        let config = Config::load("default")?;
        let user_info = goauth::user_info(&config.access_token)?;

        println!("{}", user_info.email);

        Ok(())
    }
}
