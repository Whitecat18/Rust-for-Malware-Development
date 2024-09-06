use chrono::{NaiveDateTime, Utc};
use std::env;
use std::process::exit;

const TIME_ATTACK_IN_DAYS: f64 = 1.0;

fn time_when_compiled() -> NaiveDateTime {
    let build_date = env!("BUILD_DATE");
    let build_time = env!("BUILD_TIME");

    let datetime_str = format!("{} {}", build_date, build_time);

    NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse build date and time")
}

fn main() {
    let current_time = Utc::now().naive_utc();

    let build_time = time_when_compiled();

    let diff_time = current_time.signed_duration_since(build_time).num_seconds();

    let time_to_wait = (TIME_ATTACK_IN_DAYS * 24.0 * 60.0 * 60.0) as i64;

    if diff_time > time_to_wait {
        println!("Time of attack!");
        exit(-1);
    } else {
        println!(
            "Time left to attack: {}",
            time_to_wait - diff_time
        );
    }
}
