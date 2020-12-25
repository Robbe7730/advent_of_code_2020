mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::day::Day;
use std::path::PathBuf;

pub fn run_day(
    day_num: usize,
    bench_num: usize,
    part_num: Option<usize>,
    input_file: Option<PathBuf>,
    timed: bool,
) {
    let run_part1 = match part_num {
        None => true,
        Some(x) => x == 1 || x == 0,
    };
    let run_part2 = match part_num {
        None => true,
        Some(x) => x == 2 || x == 0,
    };
    match day_num {
        1 => day01::Day01 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        2 => day02::Day02 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        3 => day03::Day03 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        4 => day04::Day04 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        5 => day05::Day05 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        6 => day06::Day06 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        7 => day07::Day07 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        8 => day08::Day08 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        9 => day09::Day09 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        10 => day10::Day10 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        11 => day11::Day11 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        12 => day12::Day12 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        13 => day13::Day13 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        14 => day14::Day14 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        15 => day15::Day15 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        16 => day16::Day16 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        17 => day17::Day17 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        18 => day18::Day18 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        19 => day19::Day19 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        20 => day20::Day20 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        21 => day21::Day21 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        22 => day22::Day22 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        23 => day23::Day23 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        24 => day24::Day24 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        25 => day25::Day25 {}.execute(day_num, bench_num, run_part1, run_part2, input_file, timed),
        _ => panic!("Day {} doesn't exist...", day_num),
    };
}

pub fn run_all_days(day_num: usize, bench_num: usize, timed: bool) {
    for day_number in 1..(day_num + 1) {
        println!("----- DAY {} -----", day_number);
        run_day(day_number, bench_num, None, None, timed);
    }
}
