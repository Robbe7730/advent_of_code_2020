use crate::day::Day;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Answer {
    bitmap: u32,
}

impl FromStr for Answer {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Answer {
            bitmap: line.bytes().map(|b| 1 << (b - 97)).fold(0, |a, x| a | x),
        })
    }
}

pub struct Day06 {}

impl Day for Day06 {
    type InputElement = Vec<Answer>;
    type Output1 = u32;
    type Output2 = u32;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input
            .iter()
            .map(|group| group.iter().fold(0, |a, x| a | x.bitmap))
            .map(|x1| x1.count_ones())
            .sum()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input
            .iter()
            .map(|group| group.iter().fold(0x7ffffff, |a, x| a & x.bitmap))
            .map(|x1| x1.count_ones())
            .sum()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .split("\n\n")
            .map(|l| {
                l.lines()
                    .map(|x| x.parse::<Answer>().expect("Invalid input"))
                    .collect()
            })
            .collect()
    }
}
