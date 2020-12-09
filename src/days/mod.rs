mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use crate::day::Day;
use std::path::PathBuf;

pub fn run_day(day_num: usize, bench_num: usize, part_num: Option<usize>, input_file: Option<PathBuf>, timed: bool) {
    let run_part1 = match part_num {
        None => true,
        Some(x) => x == 1 || x == 0
    };
    let run_part2 = match part_num {
        None => true,
        Some(x) => x == 2 || x == 0
    };
    match day_num {
        1 => day01::Day01{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        2 => day02::Day02{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        3 => day03::Day03{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        4 => day04::Day04{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        5 => day05::Day05{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        6 => day06::Day06{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        7 => day07::Day07{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        8 => day08::Day08{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        9 => day09::Day09{}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };
}

pub fn run_all_days(day_num: usize, bench_num: usize, timed: bool) {
    for day_number in 1..(day_num+1) {
        println!("----- DAY {} -----", day_number);
        run_day(day_number, bench_num, None, None, timed);
    }
}
