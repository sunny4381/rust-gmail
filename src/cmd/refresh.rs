use clap::Clap;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;
use crate::goauth;

#[derive(Clap)]
pub struct RefreshCmd {
}

impl Cmd for RefreshCmd {
    fn run(&self) -> Result<(), Error> {
        let config = Config::load("default")?;
        let token = goauth::refresh_token(&config.client_id, &config.client_secret, &config.refresh_token)?;

        let new_config = Config {
            email_from: config.email_from,
            client_id: config.client_id,
            client_secret: config.client_secret,
            access_token: token.access_token,
            expires_in: token.expires_in,
            refresh_token: config.refresh_token,
        };
        new_config.save("default")?;

        Ok(())
    }
}
