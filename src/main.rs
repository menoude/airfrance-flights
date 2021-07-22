use airfrance::*;
use {
    clokwerk::{Scheduler, TimeUnits},
    std::{thread, time::Duration},
};

fn main() {
    let date = "2021-07-28";
    println!("For {}", date);
    execute_check(date);
    let mut scheduler = Scheduler::new();
    scheduler
        .every(1.minutes())
        .run(move || execute_check(date));
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(5000));
    }
}
