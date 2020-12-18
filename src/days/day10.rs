use crate::day::Day;

pub struct Day10 {}

impl Day for Day10 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, old_input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut input = old_input.to_vec();
        input.sort();
        input.insert(0, 0);
        let (one, _two, three) = input.windows(2).map(|win| win[1] - win[0]).fold(
            (0, 0, 0),
            |(one, two, three), diff| {
                (
                    one + (diff == 1) as usize,
                    two + (diff == 2) as usize,
                    three + (diff == 3) as usize,
                )
            },
        );
        one * (three + 1)
    }

    fn solve_part2(&self, old_input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut input = old_input.to_vec();
        input.sort();
        input.insert(0, 0);
        let (_, _, ret) = input.windows(2).map(|win| win[1] - win[0]).fold(
            (0, 0, 1),
            |(three, two, one), diff| match diff {
                1 => (two, one, one + two + three),
                2 => (one, 0, one + two),
                3 => (0, 0, one),
                _ => (0, 0, 0),
            },
        );
        ret
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Self::InputElement>().expect("Invalid input"))
            .collect()
    }
}
