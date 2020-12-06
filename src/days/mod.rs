mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use crate::day::Day;

pub fn run_day(day_num: usize, bench_num: usize) {
    match day_num {
        1 => day01::Day01{}.execute(day_num, bench_num),
        2 => day02::Day02{}.execute(day_num, bench_num),
        3 => day03::Day03{}.execute(day_num, bench_num),
        4 => day04::Day04{}.execute(day_num, bench_num),
        5 => day05::Day05{}.execute(day_num, bench_num),
        6 => day06::Day06{}.execute(day_num, bench_num),
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };
}

pub fn run_all_days(day_num: usize, bench_num: usize) {
    for day_number in 1..(day_num+1) {
        println!("----- DAY {} -----", day_number);
        run_day(day_number, bench_num);
    }
}
