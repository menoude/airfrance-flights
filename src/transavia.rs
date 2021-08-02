use crate::{CheckAvailability, CLIENT};
use reqwest::header::{self as headers_list, HeaderValue};

pub struct TransaviaChecker {
    pub api_key: String,
    pub date: &'static str,
}

impl CheckAvailability for TransaviaChecker {
    fn date(&self) -> &'static str {
        self.date
    }

    fn company() -> &'static str {
        "Transavia"
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            headers_list::HOST,
            HeaderValue::from_static("api.transavia.com"),
        );
        headers.insert("apikey", self.api_key.parse().unwrap());
        headers
    }

    fn date_available(&self) -> bool {
        let response_status = loop {
            match CLIENT
                .get(format!("https://api.transavia.com/v1/flightoffers/?origin=ORY&destination=ORN&originDepartureDate={}&adults=1", self.date().replace("-", "")))
                .headers(self.build_headers())
                .send()
            {
                Ok(response) => break response.status(),
                Err(e) if e.is_request() => {
                    println!("Request error: {}", e)
                }
                Err(e) => panic!("{}", e),
            }
        };
        match response_status {
            reqwest::StatusCode::OK => true,
            _ => false,
        }
    }
}
