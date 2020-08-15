use std::fs;
use std::io::Read;

use base64;
use clap::Clap;
use lettre::SendableEmail;
use lettre_email::Email;
use serde_json::json;

use super::base::Cmd;
use crate::config::Config;
use crate::error::Error;
use crate::goauth::USER_AGENT;

const SEND_API_URL: &'static str = "https://content.googleapis.com/gmail/v1/users/me/messages/send";

// below api is listed on Google's reference, but it is hard to find the way to send message.
// const SEND_API_URL: &'static str = "https://www.googleapis.com/upload/gmail/v1/users/me/messages/send";

#[derive(Clap)]
pub struct SendCmd {
    #[clap(long)]
    pub subject: String,
    #[clap(long)]
    pub to: Vec<String>,
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

        let mut email_builder = Email::builder().from(config.email_from.as_str());
        for to in &self.to {
            email_builder = email_builder.to(to.as_str());
        }
        let email: SendableEmail = email_builder
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_subject1() {
        assert_eq!(encode_subject("日本語テスト"), "=?UTF-8?B?5pel5pys6Kqe44OG44K544OI?=");
    }

    #[test]
    fn test_encode_subject2() {
        assert_eq!(encode_subject("徳島ヴォルティス 公式の更新通知"), "=?UTF-8?B?5b6z5bO244O044Kp44Or44OG44Kj44K5IOWFrOW8j+OBruabtOaWsOmA?=\r\n =?UTF-8?B?muefpQ==?=");
    }

    #[test]
    fn test_encode_subject3() {
        assert_eq!(encode_subject("【更新通知】ヴォルティススタジアム"), "=?UTF-8?B?44CQ5pu05paw6YCa55+l44CR44O044Kp44Or44OG44Kj44K544K544K/?=\r\n =?UTF-8?B?44K444Ki44Og?=");
    }
}
