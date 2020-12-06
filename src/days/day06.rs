use crate::day::Day;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Answer {
    bitmap: u32
}

impl FromStr for Answer {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Answer {
            bitmap: line.bytes()
                        .map(|b| 1 << (b - 97))
                        .fold(0, |a,x| a | x)
        })
    }
}

pub struct Day06 {}

impl Day for Day06 {
    type InputElement = Vec<Answer>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input.iter()
              .map(|group|
                   group.iter().fold(0, |a,x| a | x.bitmap)
              ).map(|x1| {
                    // https://en.wikipedia.org/wiki/Hamming_weight
                    let mut x = x1;
                    x -= (x1 >> 1) & 0x55555555;
                    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
                    x = (x + (x >> 4)) & 0x0f0f0f0f;
                    ((x.wrapping_mul(0x01010101)) >> 24) as usize
              }).sum()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input.iter()
              .map(|group|
                   group.iter().fold(0x7ffffff, |a,x| a & x.bitmap)
              ).map(|x1| {
                    // https://en.wikipedia.org/wiki/Hamming_weight
                    let mut x = x1;
                    x -= (x1 >> 1) & 0x55555555;
                    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
                    x = (x + (x >> 4)) & 0x0f0f0f0f;
                    ((x.wrapping_mul(0x01010101)) >> 24) as usize
              }).sum()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.split("\n\n")
            .map(|l| l.lines()
                .map(|x|
                      x.parse::<Answer>()
                      .expect("Invalid input")
                ).collect()
            ).collect()
    }
}
