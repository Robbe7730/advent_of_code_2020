use crate::day::Day;

use std::str::FromStr;

pub struct Password {
    min: usize,
    max: usize,
    letter: u8,
    password: Vec<u8>,
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let line_split: Vec<&str> = line.split(|x| x == '-' || x == ':' || x == ' ').collect();

        Ok(Password {
            min: line_split[0].parse()?,
            max: line_split[1].parse()?,
            letter: line_split[2].bytes().next().expect("No char in letter"),
            password: line_split[4].bytes().collect(),
        })
    }
}

pub struct Day02 {}

impl Day for Day02 {
    type InputElement = Password;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input
            .into_iter()
            .filter(|pw| {
                let mut num = 0;
                for letter in &pw.password {
                    if *letter == pw.letter {
                        num += 1;
                        if num > pw.max {
                            return false;
                        }
                    }
                }
                num >= pw.min
            })
            .count()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input
            .into_iter()
            .filter(|pw| {
                (pw.password[pw.min - 1] == pw.letter) ^ (pw.password[pw.max - 1] == pw.letter)
            })
            .count()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Password>().expect("Invalid input"))
            .collect()
    }
}
