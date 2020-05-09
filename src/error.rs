use std::env;
use std::fmt;
use std::io;

use lettre;
use lettre_email;
use serde_json;

#[derive(Debug)]
pub enum Error {
    UnknownCommandError,
    EnvError(env::VarError),
    IoError(io::Error),
    ConfigError(String),
    SerdeError(serde_json::error::Error),
    LettreEmailError(lettre_email::error::Error),
    LettreSmtpError(lettre::smtp::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownCommandError => write!(f, "Unknown Command"),
            Error::EnvError(ref err) => write!(f, "Env error: {}", err),
            Error::IoError(ref err) => write!(f, "IO error: {}", err),
            Error::ConfigError(ref msg) => write!(f, "Config error: {}", msg),
            Error::SerdeError(ref err) => write!(f, "Serde error: {}", err),
            Error::LettreEmailError(ref err) => write!(f, "Lettre Email error: {}", err),
            Error::LettreSmtpError(ref err) => write!(f, "Lettre Smtp error: {}", err),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeError(err)
    }
}

impl From<lettre_email::error::Error> for Error {
    fn from(err: lettre_email::error::Error) -> Error {
        Error::LettreEmailError(err)
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(err: lettre::smtp::error::Error) -> Error {
        Error::LettreSmtpError(err)
    }
}
