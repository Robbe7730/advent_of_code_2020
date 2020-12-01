mod day;
mod days;

extern crate itertools;

use std::env;

use days::run_day;
use days::run_all_days;

fn main() {
    let day_arg = env::args().nth(1);
    if let Some(day) = day_arg {
        if day == "all" {
            run_all_days();
        } else if let Ok(day_number) = day.parse() {
            run_day(day_number);
        } else {
            println!("Invalid argument {}", day);
        }
    } else {
        println!("Usage:\n  `{0} <day_number>` for a specific day\n  `{0} all` to run all days",
                 env::args()
                        .next()
                        .unwrap_or_else(|| String::from("advent_of_code")));
    }
}
