use flight_checker::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
};

fn main() {
    init();

    let air_france_api_key = std::env::var("API_KEY").expect("Set API_KEY env var");
    let transavia_api_key = std::env::var("TRANSAVIA_API_KEY").expect("Set API_KEY env var");

    let checks: Vec<Box<dyn CheckAvailability + Send + Sync>> = vec![
        Box::new(AirFranceChecker {
            date: "2021-08-25",
            api_key: air_france_api_key,
        }),
        Box::new(TransaviaChecker {
            date: "2021-08-12",
            api_key: transavia_api_key.clone(),
        }),
        Box::new(TransaviaChecker {
            date: "2021-08-26",
            api_key: transavia_api_key,
        }),
    ];

    let mut scheduler = Scheduler::new();
    scheduler.every(45.seconds()).run(move || {
        for check in &checks {
            check.execute_check();
        }
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
