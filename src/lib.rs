use reqwest::header::{self as headers_list, HeaderValue};

#[macro_use]
extern crate serde_derive;

pub fn build_headers_map() -> reqwest::header::HeaderMap {
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
    headers.insert(
        "api-key",
        std::env::var("API_KEY")
            .expect("env var missing")
            .parse()
            .unwrap(),
    );
    headers
}

pub fn date_availability(
    client: &reqwest::blocking::Client,
    headers: reqwest::header::HeaderMap,
    date: &'static str,
) -> bool {
    let data = PostData {
        commercial_cabins: vec!["ALL"],
        passenger_count: PassengerCount { adult: 1 },
        requested_connections: vec![RequestedConnections {
            departure_date: date,
            origin: Place {
                airport: Airport { code: "CDG" },
            },
            destination: Place {
                airport: Airport { code: "ALG" },
            },
        }],
    };

    let response = client
        .post("https://api.airfranceklm.com/opendata/offers/v1/available-offers")
        .body(serde_json::to_vec(&data).expect("Can't serialize data"))
        .headers(headers)
        .send()
        .unwrap();
    let body: AirFranceResponse = serde_json::from_reader(response).unwrap();
    match body {
        AirFranceResponse::Warning { .. } => false,
        AirFranceResponse::Itineraries { .. } => true,
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
