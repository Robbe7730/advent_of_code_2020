use crate::day::Day;

use std::collections::VecDeque;
use std::time::SystemTime;

pub struct Day23 {}

impl Day for Day23 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let state = self.play(input, 100);

        let start_pos = state.iter().position(|x| *x == 1).unwrap() + 1;
        let mut ret = 0;
        for i in 0..8 {
            ret = ret * 10 + state[(start_pos + i) % 9];
        }
        ret
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut new_input = input.clone();
        for i in 10..=1_000_000 {
            new_input.push(i);
        }

        let state = self.play(&new_input, 10_000_000);

        let start_pos = state.iter().position(|x| *x == 1).unwrap() + 1;
        state[(start_pos + 1) % 1_000_000] * state[(start_pos + 2) % 1_000_000]
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .trim()
            .bytes()
            .map(|x| (x - b'0') as usize)
            .collect()
    }
}

impl Day23 {
    fn play(&self, input: &Vec<usize>, num_turns: usize) -> VecDeque<usize> {
        let mut curr_state = VecDeque::from(input.clone());
        let cup_count = 9;
        let mut last_time = SystemTime::now();

        for turn in 0..num_turns {
            if turn % (num_turns / 100) == 0 {
                println!(
                    "{}% complete, took {} ms",
                    turn / (num_turns / 100),
                    last_time.elapsed().unwrap().as_millis()
                );
                last_time = SystemTime::now();
            }

            // Find destination cup (part 1)
            let mut dest_cup_label = curr_state[0] - 1; // O(1)

            // Rotate
            curr_state.rotate_left(1); // O(1)

            // Pick up three cups
            let first_cup = curr_state.pop_front().unwrap(); // O(1)
            let second_cup = curr_state.pop_front().unwrap(); // O(1)
            let third_cup = curr_state.pop_front().unwrap(); // O(1)

            // Find destination cup (part 2)
            let mut dest_pos_opt = curr_state.iter().position(|x| *x == dest_cup_label); // O(n)
            while dest_pos_opt == None {
                dest_cup_label = (dest_cup_label + cup_count) % (cup_count + 1); // O(1)
                dest_pos_opt = curr_state.iter().position(|x| *x == dest_cup_label); // O(n)
            }

            let dest_pos = dest_pos_opt.unwrap(); // O(n)

            // Place cups
            curr_state.insert((dest_pos + 1) % cup_count, third_cup); // O(n)
            curr_state.insert((dest_pos + 1) % cup_count, second_cup); // O(n)
            curr_state.insert((dest_pos + 1) % cup_count, first_cup); // O(n)
        }
        curr_state
    }
}
