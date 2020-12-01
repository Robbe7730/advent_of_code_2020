mod day01;

use crate::day::Day;

pub fn run_day(day_num: usize) {
    let day = match day_num {
        1 => day01::Day01{},
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };

    day.solve(day_num);
}
