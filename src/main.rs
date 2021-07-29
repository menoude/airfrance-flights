use flight_checker::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
};

fn main() {
    init();

    let air_france_api_key = std::env::var("API_KEY").expect("Set API_KEY env var");

    let air_france_aug_fourth = AirFranceChecker {
        date: "2021-08-04",
        api_key: air_france_api_key.clone(),
    };
    let air_france_aug_eleventh = AirFranceChecker {
        date: "2021-08-11",
        api_key: air_france_api_key,
    };

    air_france_aug_fourth.execute_check();
    air_france_aug_eleventh.execute_check();

    let mut scheduler = Scheduler::new();
    scheduler.every(45.seconds()).run(move || {
        air_france_aug_fourth.execute_check();
        air_france_aug_eleventh.execute_check();
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
