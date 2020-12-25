use crate::day::Day;

pub struct Day25 {}

impl Day for Day25 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let subject_number = 7;

        let door_key = input[0];
        let card_key = input[1];

        let mut loop_count = 0;
        let mut value = 1;

        while value != door_key && value != card_key {
            value *= subject_number;
            value %= 20201227;
            loop_count += 1;
        }

        let subject_number_2 = if value == card_key {
            door_key
        } else {
            card_key
        };

        let mut encryption_key = 1;

        for _ in 0..loop_count {
            encryption_key *= subject_number_2;
            encryption_key %= 20201227;
        }
        encryption_key
    }

    fn solve_part2(&self, _input: &Vec<Self::InputElement>) -> Self::Output2 {
        0
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse().expect("Invalid input"))
            .collect()
    }
}
