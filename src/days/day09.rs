use crate::day::Day;

use itertools::iproduct;

pub struct Day09 {}

impl Day for Day09 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let preamble_count = 25;
        *input
            .windows(preamble_count + 1)
            .map(|slice| {
                let target: &usize = slice.last().expect("Unreachable statement");
                (
                    iproduct!(slice, slice)
                        .filter(move |(x, y)| x != y && *x + *y == *target)
                        .map(|(x, y)| x + y)
                        .next(),
                    target,
                )
            })
            .filter(|(v, _)| *v == None)
            .next()
            .expect("No solution")
            .1
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let target = self.solve_part1(input);
        let mut curr_sum = 0;
        let mut start = 0;
        let mut end = 0;
        for (i, last) in input.iter().enumerate() {
            curr_sum += last;
            end = i;
            while curr_sum > target {
                curr_sum -= input[start];
                start += 1;
            }
            if curr_sum == target {
                break;
            }
        }
        let (min, max) = &input[start..=end]
            .iter()
            .fold((usize::MAX, usize::MIN), |(c_min, c_max), val| {
                (c_min.min(*val), c_max.max(*val))
            });
        min + max
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Self::InputElement>().expect("Invalid input"))
            .collect()
    }
}
