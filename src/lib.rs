mod air_france;
// mod asl;

pub use air_france::AirFranceChecker;

use termion::color;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
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
    pub static ref RECEIVER_PHONE_NUMBERS: Vec<String> = std::env::var("RECEIVER_PHONE_NUMBERS")
        .expect("Set RECEIVER_PHONE_NUMBERS env var")
        .split(", ")
        .map(|str| str.to_owned())
        .collect();
    pub static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn init() {
    lazy_static::initialize(&TWILIO_ACCOUNT_SID);
    lazy_static::initialize(&TWILIO_AUTH_TOKEN);
    lazy_static::initialize(&SENDER_PHONE_NUMBER);
    lazy_static::initialize(&RECEIVER_PHONE_NUMBERS);
}

pub trait CheckAvailability {
    fn date(&self) -> &'static str;
    fn company() -> &'static str;

    fn execute_check(&self) {
        println!("For {}", self.date());
        let time = chrono::Local::now().time().format("%H:%M:%S").to_string();
        match self.date_available() {
            false => println!(
                "{}{} | No flight available.{}",
                color::Fg(color::Red),
                time,
                color::Fg(color::Reset)
            ),
            true => {
                println!(
                    "{}{} | Some flight available!!!!!!!!!!!{}",
                    color::Fg(color::Green),
                    time,
                    color::Fg(color::Reset)
                );
                for receiver in RECEIVER_PHONE_NUMBERS.iter() {
                    CLIENT
                        .post(format!(
                            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                            TWILIO_ACCOUNT_SID.as_str()
                        ))
                        .basic_auth(
                            TWILIO_ACCOUNT_SID.as_str(),
                            Some(TWILIO_AUTH_TOKEN.as_str()),
                        )
                        .body(
                            serde_json::to_vec(&TwilioPayload {
                                body: format!(
                                    "Un billet disponible chez {} pour le {}, appelle Mennad !",
                                    Self::company(),
                                    self.date()
                                ),
                                from: SENDER_PHONE_NUMBER.as_str(),
                                to: receiver,
                            })
                            .expect("Can't serialize data"),
                        )
                        .send()
                        .unwrap();
                }
            }
        }
    }
    fn date_available(&self) -> bool;
    fn build_headers(&self) -> reqwest::header::HeaderMap;
}
// curl -X POST https://api.twilio.com/2010-04-01/Accounts/$TWILIO_ACCOUNT_SID/Messages.json --data-urlencode "Body=premier test de sms" --data-urlencode "From=+13235537695" --data-urlencode "To=+33751601633" -u $TWILIO_ACCOUNT_SID:$TWILIO_AUTH_TOKEN
