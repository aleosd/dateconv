extern crate chrono;
extern crate regex;

use std::env;
use chrono::prelude::*;
use std::process;
use regex::Regex;


fn dtts(date_str: &str, with_time: bool) -> i64 {
    if with_time {
        let parsed_result = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap();
        return parsed_result.timestamp();
    } else {
        let native_time = NaiveTime::from_hms(0, 0, 0);
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let datetime = date.and_time(native_time);
        return datetime.timestamp();
    }
}


fn tsdt(unix_timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    return naive.format("%Y-%m-%d %H:%M:%S").to_string();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error! Provide exactly one date to convert");
        process::exit(1);
    }

    let given_date = &args[1];
    if given_date.parse::<i64>().is_ok() {
        let datetime = tsdt(given_date.parse::<i64>().unwrap());
        println!("{}", datetime);
    } else {
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        let datetime_re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();

        if date_re.is_match(given_date) {
            let unix_timestamp = dtts(given_date, false);
            println!("{}", unix_timestamp);
        } else if datetime_re.is_match(given_date) {
            let unix_timestamp = dtts(given_date, true);
            println!("{}", unix_timestamp);
        } else {
            println!("Unsupported datetime format: {}", given_date);
            process::exit(1);
        }
    }
}
