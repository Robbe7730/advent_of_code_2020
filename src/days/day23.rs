use crate::day::Day;

pub struct Day23 {}

impl Day for Day23 {
    type InputElement = usize;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let state = self.play(input, 100);

        let mut curr_pos = state[1];
        let mut ret = 0;
        for _ in 0..8 {
            ret = ret * 10 + curr_pos;
            curr_pos = state[curr_pos];
        }
        ret
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut new_input = input.clone();
        for i in 10..=1_000_000 {
            new_input.push(i);
        }

        let next = self.play(&new_input, 10_000_000);

        // 999872003420 -> too high
        next[1] * next[next[1]]
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
    fn play(&self, input: &Vec<usize>, num_turns: usize) -> Vec<usize> {
        let cup_count = input.len();

        let mut next: Vec<usize> = vec![0; cup_count + 1];

        for window in input.windows(2) {
            next[window[0]] = window[1];
        }
        next[*input.last().unwrap()] = input[0];

        let mut curr_cup = input[0];

        for _ in 0..num_turns {
            // Pick up three cups
            let first_cup = next[curr_cup];
            let second_cup = next[first_cup];
            let third_cup = next[second_cup];
            next[curr_cup] = next[third_cup];

            // Find the destination cup
            let mut dest_cup;
            if curr_cup == 1 {
                dest_cup = cup_count;
            } else {
                dest_cup = curr_cup - 1;
            }
            while dest_cup == first_cup || dest_cup == second_cup || dest_cup == third_cup {
                if dest_cup == 1 {
                    dest_cup = cup_count;
                } else {
                    dest_cup -= 1;
                }
            }

            // Place cups back
            next[third_cup] = next[dest_cup];
            next[dest_cup] = first_cup;

            // Select next cup
            curr_cup = next[curr_cup];
        }

        next
    }
}
