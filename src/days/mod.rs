mod day01;

use crate::day::Day;

pub fn run_day(day_num: usize, bench_num: usize) {
    let day = match day_num {
        1 => day01::Day01{},
        _ => panic!("Day {} hasn't been solved (yet)", day_num),
    };

    day.solve(day_num);

    if bench_num > 0 {
        day.bench(bench_num, day_num);
    }
}

pub fn run_all_days(day_num: usize, bench_num: usize) {
    for day_number in 1..(day_num+1) {
        println!("----- DAY {} -----", day_number);
        run_day(day_number, bench_num);
    }
}
