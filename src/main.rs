use flight_checker::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
};

fn main() {
    init();

    let air_france_checker = flight_checker::AirFranceChecker {
        date: "2021-07-28",
        api_key: std::env::var("API_KEY").expect("Set API_KEY env var"),
    };

    air_france_checker.execute_check();

    let mut scheduler = Scheduler::new();
    scheduler.every(1.minutes()).run(move || {
        air_france_checker.execute_check();
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
