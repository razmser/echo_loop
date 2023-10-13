use chrono::Utc;
use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let period = args.get(1);
    let period = period.map_or(Option::None, |str| {
        Option::from(str.parse::<u64>().expect("bad integer"))
    });
    let period = period.map(|millis| Duration::from_millis(millis));
    let mut count = 0;
    let mut last_run = Instant::now();
    loop {
        println!("{} log line {}", Utc::now(), count);
        count += 1;

        if period.is_none() {
            continue;
        }
        let period = period.unwrap();
        let elapsed = last_run.elapsed();
        if elapsed < period {
            thread::sleep(period - elapsed);
        }
        last_run = Instant::now();
    }
}
