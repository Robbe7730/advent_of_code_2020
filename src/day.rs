use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::time::SystemTime;
use std::convert::TryInto;
use std::path::PathBuf;

use rand::seq::SliceRandom;

pub trait Day {
    type InputElement;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, content: String) -> Vec<Self::InputElement>;
    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1;
    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2;

    fn execute(&self, day_num: usize, bench_num: usize, run_part1: bool, run_part2: bool, input_file: Option<PathBuf>, timed: bool) {
        let filename = match input_file {
            None => format!("input/day{}", day_num),
            Some(p) => p.into_os_string().into_string().expect("Invalid file path"),
        };
        let input_unparsed = self.read_input(&filename);

        if let Ok(input) = input_unparsed {
            let input = self.parse_input(input);
            if run_part1 {
                if timed {
                    let (sol, time) = self.solve_part1_timed(&input);
                    println!("Part 1: {} (took {}ns)", sol, time);
                } else {
                    let sol = self.solve_part1(&input);
                    println!("Part 1: {}", sol);
                }
            }
            if run_part2 {
                if timed {
                    let (sol, time) = self.solve_part2_timed(&input);
                    println!("Part 2: {} (took {}ns)", sol, time);
                } else {
                    let sol = self.solve_part2(&input);
                    println!("Part 2: {}", sol);
                }
            }
        } else {
            println!("No input file found");
        }

        if bench_num > 0 {
           self.bench(bench_num, &filename);
        }

    }

    fn bench(&self, bench_num: usize, input_filename: &String) {
        let input_unparsed = self.read_input(input_filename);
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

    fn solve_part1_timed(&self, input: &Vec<Self::InputElement>) -> (Self::Output1, u128) {
        let timer = SystemTime::now();
        let ret = self.solve_part1(input);
        let time_taken = timer.elapsed().unwrap();
        (ret, time_taken.as_nanos())
    }

    fn solve_part2_timed(&self, input: &Vec<Self::InputElement>) -> (Self::Output2, u128) {
        let timer = SystemTime::now();
        let ret = self.solve_part2(input);
        let time_taken = timer.elapsed().unwrap();
        (ret, time_taken.as_nanos())
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

    fn read_input(&self, filename: &String) -> Result<String, Error> {
        let mut ret = String::new();
        let mut f = File::open(filename)?;
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
