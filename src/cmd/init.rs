use std::io::{self,BufRead,Write};
use clap::Clap;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;

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

#[derive(Clap)]
pub struct InitCmd {
    pub username: String,
    #[clap(long)]
    pub password: Option<String>,
}

impl Cmd for InitCmd {
    fn run(&self) -> Result<(), Error> {
        let password: String = match self.password {
            Some(ref password) => String::from(password),
            _ => read_from_stdin("Password")?,
        };

        let config = Config {
            username: String::from(&self.username),
            password: password,
        };
        config.save("default")?;

        Ok(())
    }
}
