use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::time::SystemTime;

use std::convert::TryInto;

pub trait Day {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, content: String) -> Self::Input;
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1;
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2;

    fn execute(&self, day_num: usize, bench_num: usize) {
        self.solve(day_num);

        if bench_num > 0 {
           self.bench(bench_num, day_num);
        }

    }

    fn solve(&self, day_number: usize) {
        let test_input_unparsed = self.read_test_input(day_number);
        let input_unparsed = self.read_input(day_number);

        if let Ok(input) = test_input_unparsed {
            let input = self.parse_input(input);
            println!("SAMPLE INPUT:");
            self.solve_part1_timed(&input);
            self.solve_part2_timed(&input);
        } else {
            println!("No test file found");
        }

        if let Ok(input) = input_unparsed {
            let input = self.parse_input(input);
            println!("FULL INPUT:");
            self.solve_part1_timed(&input);
            self.solve_part2_timed(&input);
        } else {
            println!("No input file found");
        }
    }

    fn bench(&self, bench_num: usize, day_number: usize) {
        let input_unparsed = self.read_input(day_number);
        if let Ok(input) = input_unparsed {
            let input = self.parse_input(input);
            println!("BENCHMARK");
            self.bench_part1(&input, bench_num.try_into().expect(""));
            self.bench_part2(&input, bench_num.try_into().expect(""));
        } else {
            println!("No input file found");
        }
    }

    fn solve_part1_timed(&self, input: &Self::Input) {
        let timer = SystemTime::now();
        let ret = self.solve_part1(input);
        let time_taken = timer.elapsed().unwrap();
        println!("Part 1: {} (took {}ns)", ret, time_taken.as_nanos());
    }

    fn solve_part2_timed(&self, input: &Self::Input) {
        let timer = SystemTime::now();
        let ret = self.solve_part2(input);
        let time_taken = timer.elapsed().unwrap();
        println!("Part 2: {} (took {}ns)", ret, time_taken.as_nanos());
    }

    fn bench_part1(&self, input: &Self::Input, n: u32) {
        let timer = SystemTime::now();
        for _ in 0..n {
            self.solve_part1(input);
        }
        let time_taken = timer.elapsed().unwrap();
        let avg_time = (time_taken / n).as_nanos();
        println!("Part 1 averaged {}ns over {} iterations", avg_time, n);
    }

    fn bench_part2(&self, input: &Self::Input, n: u32) {
        let timer = SystemTime::now();
        for _ in 0..n {
            self.solve_part2(input);
        }
        let time_taken = timer.elapsed().unwrap();
        let avg_time = (time_taken / n).as_nanos();
        println!("Part 2 averaged {}ns over {} iterations", avg_time, n);
    }

    fn read_input(&self, day_number: usize) -> Result<String, Error> {
        let mut ret = String::new();
        let mut f = File::open(format!("input/day{}", day_number))?;
        f.read_to_string(&mut ret).expect("Could not read file");
        Ok(ret)
    }

    fn read_test_input(&self, day_number: usize) -> Result<String, Error> {
        let mut ret = String::new();
        let mut f = File::open(format!("input/sample_day{}", day_number))?;
        f.read_to_string(&mut ret).expect("Could not read file");
        Ok(ret)
    }
}
