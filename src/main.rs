use airfrance::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
};

fn main() {
    let client = reqwest::blocking::Client::new();
    let headers = build_headers_map();
    let date = "2021-07-28";
    let mut scheduler = Scheduler::new();
    scheduler.every(1.minutes()).run(move || {
        println!(
            "For {} -> {}",
            date,
            match date_availability(&client, headers.clone(), date) {
                false => "No flight available",
                true => "Some flight available",
            }
        )
    });
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
