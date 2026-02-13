#![allow(unused)]
use chrono;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
fn main() {
    // let mut time = Duration::from_secs(12);
    // time += Duration::from_micros(42);
    // let only_secs = time.as_secs();
    // println!("The total seconds is {}", only_secs);

    // loop {
    //     println!("{}", std::time::UNIX_EPOCH.elapsed().unwrap().as_secs());
    //     sleep(std::time::Duration::from_secs(1));
    // }

    // #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    // pub(crate) struct Timespec {
    //     tv_sec: i64,
    //     tv_nsec: Nanoseconds,
    // }
    // loop {
    //     println!("Time: {:?}", std::time::Instant::now());
    // }

    loop {
        println!("Time local: {}", chrono::Local::now());
        println!("Time utc: {}", chrono::Utc::now());

        sleep(Duration::from_secs(1));
        let time = Utc.with_ymd_and_hms(2005, 7, 13, 1, 59, 1);
        let tiem = Local.with_ymd_and_hms(2005, 7, 13, 1, 59, 1);
        println!("Birthday utc: {}", time.unwrap());
        println!("Birthday local: {}", tiem.unwrap());

        println!(
            "year month and hour minutes: {}",
            time.unwrap().format("%Y-%m %H:%M")
        );

        println!("Born on {}", time.unwrap().format("%a %b"));
    }
}
