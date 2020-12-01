mod day01;

use crate::day::Day;

const IMPLEMENTED_DAYS: [usize; 1] = [1];

pub fn run_day(day_num: usize) {
    let day = match day_num {
        1 => day01::Day01{},
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };

    day.solve(day_num);
}

pub fn run_all_days() {
    for day_number in IMPLEMENTED_DAYS.iter() {
        println!("----- DAY {} -----", *day_number);
        run_day(*day_number);
    }
}
