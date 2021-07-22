use airfrance::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
    termion::color,
};

fn main() {
    let client = reqwest::blocking::Client::new();
    let headers = build_headers_map();
    let date = "2021-07-28";
    println!("For {}", date);
    let mut scheduler = Scheduler::new();
    scheduler.every(1.minutes()).run(move || {
        let time = chrono::Local::now().time().format("%H:%M:%S").to_string();
        match date_availability(&client, headers.clone(), date) {
            false => println!(
                "{}{} | No flight available.{}",
                color::Fg(color::Red),
                time,
                color::Fg(color::Reset)
            ),
            true => println!(
                "{}{} | Some flight available!!!!!!!!!!!{}",
                color::Fg(color::Green),
                time,
                color::Fg(color::Reset)
            ),
        }
    });
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
