extern crate clap;
extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use std::process;
use regex::Regex;


fn dtts(date_str: &str, with_time: bool, with_sec: bool) -> i64 {
    if with_time {
        let parsed_result = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M").unwrap();
        return parsed_result.timestamp();
    } else if with_sec {
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
    let args = clap::App::new("Unixtime date converter")
                .version("0.1.1")
                .author("Osadchuk A <osdalex@gmail.com>")
                .about("Converts unix timestamp to YYYY-MM-DD and vice versa")
                .arg(clap::Arg::with_name("date")
                     .help("Date to parse")
                     .required(true)
                     .index(1))
                .get_matches();

    let given_date = args.value_of("date").unwrap();
    if given_date.parse::<i64>().is_ok() {
        let datetime = tsdt(given_date.parse::<i64>().unwrap());
        println!("{}", datetime);
    } else {
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        let datetime_re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$").unwrap();
        let datetimesec_re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();

        if date_re.is_match(given_date) {
            let unix_timestamp = dtts(given_date, false, false);
            println!("{}", unix_timestamp);
        } else if datetime_re.is_match(given_date) {
            let unix_timestamp = dtts(given_date, true, false);
            println!("{}", unix_timestamp);
        } else if datetimesec_re.is_match(given_date) {
            let unix_timestamp = dtts(given_date, false, true);
            println!("{}", unix_timestamp);
        } else {
            println!("Unsupported datetime format: \"{}\", supported are YYYY-MM-DD, YYYY-MM-DD HH:MM and YYYY-MM-DD HH:MM:SS", given_date);
            process::exit(1);
        }
    }
}
