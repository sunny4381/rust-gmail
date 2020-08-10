use std::io::{self,BufRead,Write};
use clap::Clap;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;
use crate::goauth::{auth_url, auth_token};

fn prompt(label: &str) -> Result<(), Error> {
    print!("put your {}: ", label);
    io::stdout().flush()?;
    return Ok(());
}

fn read_from_stdin(label: &str) -> Result<String, Error> {
    loop {
        prompt(label)?;

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = input.unwrap()?;
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

fn read_code(client_id: &str) -> Result<String, Error> {
    println!("visit {}", auth_url(client_id));

    loop {
        prompt("code")?;

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = input.unwrap()?;
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

#[derive(Clap)]
pub struct InitCmd {
    pub email_from: String,
    #[clap(long)]
    pub client_id: Option<String>,
    #[clap(long)]
    pub client_secret: Option<String>,
}

impl Cmd for InitCmd {
    fn run(&self) -> Result<(), Error> {
        let client_id: String = match self.client_id {
            Some(ref client_id) => String::from(client_id),
            _ => read_from_stdin("client id")?,
        };

        let client_secret: String = match self.client_secret {
            Some(ref client_secret) => String::from(client_secret),
            _ => read_from_stdin("client secret")?,
        };

        let code = read_code(&client_id)?;
        let token = auth_token(&client_id, &client_secret, &code)?;

        let config = Config {
            email_from: String::from(&self.email_from),
            client_id: client_id,
            client_secret: client_secret,
            access_token: token.access_token,
            expires_in: token.expires_in,
            refresh_token: token.refresh_token.unwrap()
        };
        config.save("default")?;

        Ok(())
    }
}
