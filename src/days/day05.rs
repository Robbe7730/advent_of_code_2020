use crate::day::Day;

use std::collections::HashSet;
use std::iter::FromIterator;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Seat {
    seat_id: usize
}

impl FromStr for Seat {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Seat {
            seat_id: line.bytes()
                .enumerate()
                .map(|(i, x)| ((x == 'B' as u8 || x == 'R' as u8) as usize) << (9-i))
                .sum()
        })

    }
}

pub struct Day05 {}

impl Day for Day05 {
    type InputElement = Seat;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input.into_iter()
            .map(|x| x.seat_id)
            .max()
            .expect("No result")
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let set: HashSet<usize> = HashSet::from_iter(input.into_iter()
                                        .map(|x| x.seat_id));
        (1..1024_usize)
                 .filter(|x| 
                         !set.contains(x) &&
                         set.contains(&(x+1)) &&
                         set.contains(&(x-1))
                 ).next().expect("No result")
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.lines().map(|x| x.parse::<Self::InputElement>().expect("Invalid input")).collect()
    }
}
