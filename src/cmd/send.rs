use std::fs;
use std::io::Read;

use base64;
use clap::Clap;
use lettre::SendableEmail;
use lettre_email::Email;
use serde_json::json;

use super::base::{Cmd, retry};
use crate::config::Config;
use crate::error::Error;
use crate::goauth::USER_AGENT;
use crate::cmd::refresh::RefreshCmd;

const SEND_API_URL: &'static str = "https://content.googleapis.com/gmail/v1/users/me/messages/send";
const MAX_TRIES: u16 = 3;

// below api is listed on Google's reference, but it is hard to find the way to send message.
// const SEND_API_URL: &'static str = "https://www.googleapis.com/upload/gmail/v1/users/me/messages/send";

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
        let run1 = || -> Result<(), Error> {
            let config = Config::load("default")?;

            let text = fs::read_to_string(&self.body_file)?;

            let email: SendableEmail = Email::builder()
                .from(config.email_from.as_str())
                .to(self.to.as_str())
                .subject(encode_subject(self.subject.as_str()))
                .text(text)
                .build()?
                .into();

            let mut message_content = String::new();
            email.message().read_to_string(&mut message_content)?;

            let base64_content: String = base64::encode_config(message_content.as_bytes(), base64::URL_SAFE);

            let request_json: serde_json::value::Value = json!({
                "raw": base64_content
            });

            let access_token = config.access_token;

            let client = reqwest::blocking::Client::new();
            let req = client.post(SEND_API_URL)
                .bearer_auth(access_token)
                .header(reqwest::header::USER_AGENT, USER_AGENT)
                .json(&request_json);

            let res = req.send()?;
            if !res.status().is_success() {
                return Err(Error::from(res));
            }

            Ok(())
        };

        let before_retry = || -> Result<(), Error> {
            RefreshCmd {}.run()
        };

        retry(MAX_TRIES, run1, before_retry)
    }
}
