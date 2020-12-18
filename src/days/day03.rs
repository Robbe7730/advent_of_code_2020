use crate::day::Day;

use std::str::FromStr;

pub struct Row {
    trees: Vec<bool>,
}

impl FromStr for Row {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Row {
            trees: line.bytes().map(|x| x == ('#' as u8)).collect(),
        })
    }
}

pub struct Day03 {}

impl Day for Day03 {
    type InputElement = Row;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let w = input[0].trees.len();
        input
            .into_iter()
            .fold((0, 0), |(x, c), l| (x + 3, c + (l.trees[x % w] as usize)))
            .1
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let w = input[0].trees.len();
        let sol_11 = input
            .into_iter()
            .fold((0, 0), |(x, c), l| (x + 1, c + (l.trees[x % w] as usize)))
            .1;
        let sol_31 = input
            .into_iter()
            .fold((0, 0), |(x, c), l| (x + 3, c + (l.trees[x % w] as usize)))
            .1;
        let sol_51 = input
            .into_iter()
            .fold((0, 0), |(x, c), l| (x + 5, c + (l.trees[x % w] as usize)))
            .1;
        let sol_71 = input
            .into_iter()
            .fold((0, 0), |(x, c), l| (x + 7, c + (l.trees[x % w] as usize)))
            .1;
        let sol_12 = input
            .into_iter()
            .enumerate()
            .filter(|(n, _)| n % 2 == 0)
            .fold((0, 0), |(x, c), (_, l)| {
                (x + 1, c + (l.trees[x % w] as usize))
            })
            .1;
        sol_11 * sol_31 * sol_51 * sol_71 * sol_12
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Self::InputElement>().expect("Invalid input"))
            .collect()
    }
}
