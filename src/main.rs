use airfrance::*;

fn main() {
    let client = reqwest::blocking::Client::new();
    let headers = build_headers_map();
    let date = "2021-07-28";
    println!(
        "For {} -> {}",
        date,
        match date_availability(&client, headers, date) {
            false => "No flight available",
            true => "Some flight available",
        }
    );
}
