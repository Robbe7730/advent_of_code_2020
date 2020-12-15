use crate::day::Day;

use std::collections::HashMap;

pub struct Day15 {}

impl Day for Day15 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut last_seen: HashMap<usize, usize> = HashMap::new();
        let mut i = 0;
        let (last, rest) = input.split_last().expect("Invalid input");
        for &number in rest {
            last_seen.insert(number, i);
            i += 1;
        }
        let mut last_number = *last;
        while i < 2019 {
            let new_number;
            if last_seen.contains_key(&last_number) {
                new_number = i - *last_seen.get(&last_number).expect("Unreachable");
            } else {
                new_number = 0;
            }
            last_seen.insert(last_number, i);
            i += 1;
            last_number = new_number;
        }
        last_number
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut last_seen: HashMap<usize, usize> = HashMap::new();
        let mut i = 0;
        let (last, rest) = input.split_last().expect("Invalid input");
        for &number in rest {
            last_seen.insert(number, i);
            i += 1;
        }
        let mut last_number = *last;
        while i < 29999999 {
            let new_number;
            if last_seen.contains_key(&last_number) {
                new_number = i - *last_seen.get(&last_number).expect("Unreachable");
            } else {
                new_number = 0;
            }
            last_seen.insert(last_number, i);
            i += 1;
            last_number = new_number;
        }
        last_number
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.split(',').map(|x| x.trim().parse::<Self::InputElement>().expect("Invalid input")).collect()
    }
}
