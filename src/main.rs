mod day;
mod days;

use std::env;

use days::run_day;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    run_day(day);
}
