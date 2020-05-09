use std::env;
use std::fs;
use std::io::Write;
use std::path;

use serde_json::{self, json};

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn home_dir() -> Result<path::PathBuf, env::VarError> {
        match env::var("GMAIL_HOME") {
            Ok(path) => return Ok(path::PathBuf::from(path)),
            Err(_) => (),
        };

        let home = env::var("HOME")?;
        let mut homepath = path::PathBuf::from(home);
        homepath.push(".gmail");
        return Ok(homepath);
    }

    pub fn load(profile: &str) -> Result<Config, Error> {
        let config_dir = Self::home_dir()?;
        let filepath = config_dir.as_path().join(profile);
        let file = fs::File::open(filepath)?;

        let cfg: serde_json::Value = serde_json::from_reader(file)?;
        let str_val = |key: &'static str| cfg[key].as_str().map(String::from).ok_or(Error::ConfigError(key.to_string()));
        let username = str_val("username")?;
        let password = str_val("password")?;

        return Ok(Config {
            username: username,
            password: password,
        });
    }

    pub fn save(&self, profile: &str) -> Result<(), Error> {
        let cfg = json!({
            "username": self.username,
            "password": self.password,
        });

        let config_dir = Self::home_dir()?;
        fs::create_dir_all(config_dir.as_path())?;

        let filepath = config_dir.as_path().join(profile);
        let mut file = fs::File::create(filepath)?;

        file.write_all(cfg.to_string().as_bytes())?;

        return Ok(());
    }
}
