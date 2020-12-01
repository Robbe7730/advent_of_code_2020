use crate::day::Day;

use itertools::iproduct;

#[derive(Debug)]
pub struct Day01 {}

impl Day for Day01 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        iproduct!(input, input)
            .filter(|tup| tup.0 + tup.1 == 2020)
            .map(|tup| tup.0 * tup.1)
            .next()
            .expect("No result")
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // max_allowed_value = 2020 - input.into_iter().min().expect("No values");
        let max_allowed_value = 2020;
        iproduct!(
            iproduct!(input, input)
                .filter(|tup| tup.0 + tup.1 <= max_allowed_value),
            input
        ).filter(|tup| tup.1 + tup.0.0 + tup.0.1 == 2020)
            .map(|tup| tup.1 * tup.0.0 * tup.0.1)
            .next()
            .expect("No result")
    }

    fn parse_input(&self, content: String) -> Self::Input {
        content.lines().map(|x| x.parse::<usize>().expect("Invalid input")).collect()
    }
}
