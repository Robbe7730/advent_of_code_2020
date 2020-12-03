mod day01;
mod day02;
mod day03;

use crate::day::Day;

pub fn run_day(day_num: usize, bench_num: usize) {
    match day_num {
        1 => day01::Day01{}.execute(day_num, bench_num),
        2 => day02::Day02{}.execute(day_num, bench_num),
        3 => day03::Day03{}.execute(day_num, bench_num),
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };
}

pub fn run_all_days(day_num: usize, bench_num: usize) {
    for day_number in 1..(day_num+1) {
        println!("----- DAY {} -----", day_number);
        run_day(day_number, bench_num);
    }
}
