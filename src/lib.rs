mod air_france;
mod transavia;
// mod asl;

pub use {air_france::AirFranceChecker, transavia::TransaviaChecker};

use termion::color;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TwilioPayload<'l> {
    body: String,
    to: &'l str,
    from: &'l str,
}

lazy_static! {
    pub static ref TWILIO_ACCOUNT_SID: String =
        std::env::var("TWILIO_ACCOUNT_SID").expect("Set TWILIO_AUTH_TOKEN env var");
    pub static ref TWILIO_AUTH_TOKEN: String =
        std::env::var("TWILIO_AUTH_TOKEN").expect("Set TWILIO_AUTH_TOKEN env var");
    pub static ref SENDER_PHONE_NUMBER: String =
        std::env::var("SENDER_PHONE_NUMBER").expect("Set SENDER_PHONE_NUMBER env var");
    pub static ref RECEIVERS_PHONE_NUMBERS: Vec<String> = std::env::var("RECEIVERS_PHONE_NUMBERS")
        .expect("Set RECEIVERS_PHONE_NUMBERS env var")
        .split(", ")
        .map(|str| str.to_owned())
        .collect();
    pub static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn init() {
    lazy_static::initialize(&TWILIO_ACCOUNT_SID);
    lazy_static::initialize(&TWILIO_AUTH_TOKEN);
    lazy_static::initialize(&SENDER_PHONE_NUMBER);
    lazy_static::initialize(&RECEIVERS_PHONE_NUMBERS);
}

pub trait CheckAvailability {
    fn date(&self) -> &'static str;
    fn company(&self) -> &'static str;

    fn execute_check(&self) {
        let time = chrono::Local::now().time().format("%H:%M:%S").to_string();
        match self.date_available() {
            false => println!(
                "{}{} | No {} flight available for {}.{}",
                color::Fg(color::Red),
                time,
                self.company(),
                self.date(),
                color::Fg(color::Reset)
            ),
            true => {
                println!(
                    "{}{} | Available {} flight for {}{}",
                    color::Fg(color::Green),
                    time,
                    self.company(),
                    self.date(),
                    color::Fg(color::Reset)
                );
                for receiver in RECEIVERS_PHONE_NUMBERS.iter() {
                    send_twilio_sms(self.company(), self.date(), receiver).unwrap();
                }
            }
        }
    }
    fn date_available(&self) -> bool;
    fn build_headers(&self) -> reqwest::header::HeaderMap;
}

pub fn send_twilio_sms(
    company: &str,
    date: &str,
    receiver: &'static str,
) -> reqwest::Result<reqwest::blocking::Response> {
    CLIENT
        .post(format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            TWILIO_ACCOUNT_SID.as_str()
        ))
        .basic_auth(
            TWILIO_ACCOUNT_SID.as_str(),
            Some(TWILIO_AUTH_TOKEN.as_str()),
        )
        .multipart(
            reqwest::blocking::multipart::Form::new()
                .text(
                    "Body",
                    format!(
                        "Un billet disponible chez {} pour le {}, appelle Mennad !",
                        company, date
                    ),
                )
                .text("From", SENDER_PHONE_NUMBER.as_str())
                .text("To", receiver),
        )
        .send()
}
