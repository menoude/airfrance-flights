
pub struct AslChecker<'l> {
    pub client: &'l reqwest::blocking::Client,
    pub date: &'static str,
}

impl<'l> crate::CheckAvailability for AslChecker<'l> {
    fn date(&self) -> &'static str {
        self.date
    }

    {
	"user-agent": "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0",
	"accept": "text/html, */*; q=0.01",
	"accept-language": "en-US,en;q=0.5",
	"content-type": "application/x-www-form-urlencoded; charset=UTF-8",
	"__requestverificationtoken": "uTnGwnmprDQtVnqJ6iRa19qNTFNV2wwQVZcI1e9q9PYOlUIWWBmy9CQqewu3z5Ko5LOsPda7AqEzi87uySOPlnNf4xNKPMRWTTMFZtb6hMo1",
	"x-requested-with": "XMLHttpRequest",
	"origin": "https://fo-emea.ttinteractive.com",
	"connection": "keep-alive",
	"referer": "https://fo-emea.ttinteractive.com/Zenith/FrontOffice/(S(863b79ac6abb44f695cd4e05f0af67a1))/Europeairpost/en-GB/BookingEngine/FlexibleFlightListStatic?__cnv=wPRGa",
	"cookie": "PROD_EMEA_FO=EMEA-P-WEB125; NavId=c0909618-acf0-45c8-8316-20c36e5f0670; _gcl_au=1.1.1460075077.1627258628; _ga=GA1.2.2005103193.1627258629; _gid=GA1.2.1241713646.1627258629; _gat=1; __RequestVerificationToken_L1plbml0aC9Gcm9udE9mZmljZQ2=0bS2e9Z-szmtT4FI9ej8un6FfoCvTxmekaP_nL9mNnhc77KxrcBWXWP2okHmqiUlmJFXWZhyzCglZ_d8WtxbOQZCxn8kFjp3dfe-U3PKGYw1",
	"sec-fetch-dest": "empty",
	"sec-fetch-mode": "cors",
	"sec-fetch-site": "same-origin"
      }
    fn build_headers() -> reqwest::header::HeaderMap {
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
            match self
                .client
                .post("https://api.airfranceklm.com/opendata/offers/v1/available-offers")
                .body(serde_json::to_vec(&data).expect("Can't serialize data"))
                .headers(Self::build_headers())
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
