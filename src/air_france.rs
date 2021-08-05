use crate::{CheckAvailability, CLIENT};
use reqwest::header::{self as headers_list, HeaderValue};

pub struct AirFranceChecker {
    pub api_key: String,
    pub date: &'static str,
}

impl CheckAvailability for AirFranceChecker {
    fn date(&self) -> &'static str {
        self.date
    }

    fn company(&self) -> &'static str {
        "Air France"
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
        headers_list::ACCEPT,
        HeaderValue::from_static("application/hal+json;profile=com.afklm.b2c.flightoffers.available-offers.v1;charset=utf8"));
        headers.insert(
            headers_list::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            headers_list::ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US"),
        );
        headers.insert("AFKL-TRAVEL-Host", HeaderValue::from_static("KL"));
        headers.insert("AFKL-TRAVEL-Country", HeaderValue::from_static("NL"));
        headers.insert("api-key", HeaderValue::from_str(&self.api_key).unwrap());
        headers
    }

    fn date_available(&self) -> bool {
        let data = PostData {
            commercial_cabins: vec!["ALL"],
            passenger_count: PassengerCount { adult: 1 },
            requested_connections: vec![RequestedConnections {
                departure_date: self.date(),
                origin: Place {
                    airport: Airport { code: "CDG" },
                },
                destination: Place {
                    airport: Airport { code: "ALG" },
                },
            }],
        };

        let json_respnse = loop {
            match CLIENT
                .post("https://api.airfranceklm.com/opendata/offers/v1/available-offers")
                .body(serde_json::to_vec(&data).expect("Can't serialize data"))
                .headers(self.build_headers())
                .send()
            {
                Ok(reponse) => break serde_json::from_reader(reponse).unwrap(),
                Err(e) if e.is_request() => {
                    println!("Request error: {}", e)
                }
                Err(e) => panic!("{}", e),
            }
        };
        // let body: AirFranceResponse = serde_json::from_reader(response).unwrap();
        match json_respnse {
            AirFranceResponse::Warning { .. } => false,
            AirFranceResponse::Itineraries { .. } => true,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PostData {
    pub commercial_cabins: Vec<&'static str>,
    pub passenger_count: PassengerCount,
    pub requested_connections: Vec<RequestedConnections>,
}

#[derive(Serialize)]
struct PassengerCount {
    #[serde(rename = "ADT")]
    pub adult: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestedConnections {
    pub departure_date: &'static str,
    pub origin: Place,
    pub destination: Place,
}

#[derive(Serialize)]
struct Place {
    pub airport: Airport,
}

#[derive(Serialize)]
struct Airport {
    pub code: &'static str,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AirFranceResponse {
    Warning { warnings: Vec<Warning> },
    Itineraries { itineraries: Vec<serde_json::Value> },
}

#[derive(Deserialize, Debug)]
struct Warning {
    code: usize,
    name: String,
    description: String,
}
