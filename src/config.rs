use std::env;
use std::fs;
use std::io::Write;
use std::path;

use serde_json::{self, json};

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub email_from: String,
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
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

        let email_from = cfg["email_from"].as_str().map(String::from);
        let client_id = cfg["client_id"].as_str().map(String::from);
        let client_secret = cfg["client_secret"].as_str().map(String::from);
        let access_token = cfg["access_token"].as_str().map(String::from);
        let expires_in = cfg["expires_in"].as_u64();
        let refresh_token = cfg["refresh_token"].as_str().map(String::from);

        return Ok(Config {
            email_from: email_from.ok_or(Error::ConfigError(String::from("email_from")))?,
            client_id: client_id.ok_or(Error::ConfigError(String::from("client_id")))?,
            client_secret: client_secret.ok_or(Error::ConfigError(String::from("client_secret")))?,
            access_token: access_token.ok_or(Error::ConfigError(String::from("access_token")))?,
            expires_in: expires_in.ok_or(Error::ConfigError(String::from("expires_in")))?,
            refresh_token: refresh_token.ok_or(Error::ConfigError(String::from("refresh_token")))?,
        });
    }

    pub fn save(&self, profile: &str) -> Result<(), Error> {
        let cfg = json!({
            "email_from": self.email_from,
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "access_token": self.access_token,
            "expires_in": self.expires_in,
            "refresh_token": self.refresh_token,
        });

        let config_dir = Self::home_dir()?;
        fs::create_dir_all(config_dir.as_path())?;

        let filepath = config_dir.as_path().join(profile);
        let mut file = fs::File::create(filepath)?;

        file.write_all(cfg.to_string().as_bytes())?;

        return Ok(());
    }
}
