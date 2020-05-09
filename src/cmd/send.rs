use std::fs;

use base64;
use clap::Clap;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;

#[derive(Clap)]
pub struct SendCmd {
    #[clap(long)]
    pub subject: String,
    #[clap(long)]
    pub to: String,
    pub body_file: String,
}

fn encode_subject(subject: &str) -> String {
    let slices = subject.as_bytes().chunks(3 * 14);
    let mut ret = String::new();
    for slice in slices {
        let b64 = base64::encode(slice);
        if ret.len() > 0 {
            ret.push_str("\r\n ");
        }
        ret.push_str(&format!("=?UTF-8?B?{}?=", b64));
    }

    ret
}

impl Cmd for SendCmd {
    fn run(&self) -> Result<(), Error> {
        let config = Config::load("default")?;

        let text = fs::read_to_string(&self.body_file)?;

        let email = Email::builder()
            .from(config.username.as_str())
            .to(self.to.as_str())
            .subject(encode_subject(self.subject.as_str()))
            .text(text)
            .build()?;

        let credentials = (config.username.as_str(), config.password.as_str()).into_credentials();
        let mut mailer = SmtpClient::new_simple("smtp.gmail.com")?
            .credentials(credentials)
            .smtp_utf8(true)
            .transport();

        mailer.send(email.into())?;

        Ok(())
    }
}
