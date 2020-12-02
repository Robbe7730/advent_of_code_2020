use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::time::SystemTime;

use std::convert::TryInto;
use rand::seq::SliceRandom;

pub trait Day {
    type InputElement;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, content: String) -> Vec<Self::InputElement>;
    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1;
    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2;

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
            let mut input = self.parse_input(input);
            println!("BENCHMARK");
            self.bench_part1(&input, bench_num.try_into().expect(""));
            self.bench_part2(&input, bench_num.try_into().expect(""));
            println!("BENCHMARK (shuffled inputs)");
            self.bench_part1_shuffle(&mut input, bench_num.try_into().expect(""));
            self.bench_part2_shuffle(&mut input, bench_num.try_into().expect(""));
        } else {
            println!("No input file found");
        }
    }

    fn solve_part1_timed(&self, input: &Vec<Self::InputElement>) {
        let timer = SystemTime::now();
        let ret = self.solve_part1(input);
        let time_taken = timer.elapsed().unwrap();
        println!("Part 1: {} (took {}ns)", ret, time_taken.as_nanos());
    }

    fn solve_part2_timed(&self, input: &Vec<Self::InputElement>) {
        let timer = SystemTime::now();
        let ret = self.solve_part2(input);
        let time_taken = timer.elapsed().unwrap();
        println!("Part 2: {} (took {}ns)", ret, time_taken.as_nanos());
    }

    fn bench_part1(&self, input: &Vec<Self::InputElement>, n: u32) {
        let mut time_taken = std::time::Duration::from_nanos(0);
        for _ in 0..n {
            let timer = SystemTime::now();
            self.solve_part1(input);
            time_taken += timer.elapsed().unwrap();
        }
        let avg_time = (time_taken.as_nanos() as f64) / n as f64;
        println!("Part 1 averaged {}ns over {} iterations", avg_time, n);
    }

    fn bench_part2(&self, input: &Vec<Self::InputElement>, n: u32) {
        let mut time_taken = std::time::Duration::from_nanos(0);
        for _ in 0..n {
            let timer = SystemTime::now();
            self.solve_part2(input);
            time_taken += timer.elapsed().unwrap();
        }
        let avg_time = (time_taken.as_nanos() as f64) / n as f64;
        println!("Part 2 averaged {}ns over {} iterations", avg_time, n);
    }

    fn bench_part1_shuffle(&self, input: &mut Vec<Self::InputElement>, n: u32) {
        let mut time_taken = std::time::Duration::from_nanos(0);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            input.shuffle(&mut rng);
            let timer = SystemTime::now();
            self.solve_part1(input);
            time_taken += timer.elapsed().unwrap();
        }
        let avg_time = (time_taken.as_nanos() as f64) / n as f64;
        println!("Part 1 averaged {}ns over {} iterations", avg_time, n);
    }

    fn bench_part2_shuffle(&self, input: &mut Vec<Self::InputElement>, n: u32) {
        let mut time_taken = std::time::Duration::from_nanos(0);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            input.shuffle(&mut rng);
            let timer = SystemTime::now();
            self.solve_part2(input);
            time_taken += timer.elapsed().unwrap();
        }
        let avg_time = (time_taken.as_nanos() as f64) / n as f64;
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
