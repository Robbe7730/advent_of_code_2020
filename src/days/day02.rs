use crate::day::Day;

use std::str::FromStr;

pub struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let line_split: Vec<&str> = line.split(|x| x == '-' || x == ':' || x == ' ').collect();
        
        Ok(Password {
            min: line_split[0].parse()?,
            max: line_split[1].parse()?,
            letter: line_split[2].chars().next().expect("No char in letter"),
            password: line_split[4].to_string(),
        })
    }
}

pub struct Day02 {}

impl Day for Day02 {
    type Input = Vec<Password>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input.into_iter().filter(|pw| {
            let num = pw.password.matches(pw.letter).count();
            num >= pw.min && num <= pw.max
        }).count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        input.into_iter().filter(|pw| {
            (pw.password.chars().nth(pw.min-1).expect("Not enough characters") == pw.letter) ^
            (pw.password.chars().nth(pw.max-1).expect("Not enough characters") == pw.letter)
        }).count()
    }

    fn parse_input(&self, content: String) -> Self::Input {
        content.lines().map(|x| x.parse::<Password>().expect("Invalid input")).collect()
    }
}
