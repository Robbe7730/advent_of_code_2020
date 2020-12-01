use crate::day::Day;

#[derive(Debug)]
pub struct Day01 {}

impl Day for Day01 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        for a in input {
            for b in input {
                if a + b == 2020 {
                    return a * b;
                }
            }
        }
        0
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        for a in input {
            for b in input {
                for c in input {
                    if a+b+c == 2020 {
                        return a * b * c
                    }
                }
            }
        }
        0
    }

    fn parse_input(&self, content: String) -> Self::Input {
        content.lines().map(|x| x.parse::<usize>().expect("Invalid input")).collect()
    }
}
