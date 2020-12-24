use crate::day::Day;

use std::collections::HashSet;

//    x
//   /
//  /
// +--- y

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn diff(&self) -> (isize, isize) {
        match self {
            Self::NorthEast => (1, 0),
            Self::East => (0, 1),
            Self::SouthEast => (-1, 1),
            Self::SouthWest => (-1, 0),
            Self::West => (0, -1),
            Self::NorthWest => (1, -1),
        }
    }
}

pub struct Day24 {}

impl Day for Day24 {
    type InputElement = Vec<Direction>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        self.get_floor_pattern(input).len()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut floor_pattern = self.get_floor_pattern(input);
        let all_directions = vec![
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];

        for _ in 1..=100 {
            let mut new_floor_pattern = HashSet::new();

            // Flip black -> white
            for (x, y) in floor_pattern.iter() {
                let count = all_directions
                    .iter()
                    .filter(|d| {
                        let (dx, dy) = d.diff();
                        floor_pattern.contains(&(x + dx, y + dy))
                    })
                    .count();

                if count == 1 || count == 2 {
                    new_floor_pattern.insert((*x, *y));
                }
            }

            // Flip white -> black
            let mut neighboring_whites = floor_pattern
                .iter()
                .map(|(base_x, base_y)| {
                    all_directions.iter().map(move |x| {
                        let (dx, dy) = x.diff();
                        (base_x + dx, base_y + dy)
                    })
                })
                .flatten()
                .collect::<Vec<(isize, isize)>>();
            neighboring_whites.dedup();
            for (x, y) in neighboring_whites {
                let count = all_directions
                    .iter()
                    .filter(|d| {
                        let (dx, dy) = d.diff();
                        floor_pattern.contains(&(x + dx, y + dy))
                    })
                    .count();

                if count == 2 {
                    new_floor_pattern.insert((x, y));
                }
            }

            floor_pattern = new_floor_pattern;
        }

        floor_pattern.len()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|line| {
                let new_line = format!("x{}", line);
                new_line
                    .bytes()
                    .collect::<Vec<u8>>()
                    .windows(2)
                    .filter_map(|x| match (x[0], x[1]) {
                        (b'n', b'e') => Some(Direction::NorthEast),
                        (b's', b'e') => Some(Direction::SouthEast),
                        (_, b'e') => Some(Direction::East),
                        (b'n', b'w') => Some(Direction::NorthWest),
                        (b's', b'w') => Some(Direction::SouthWest),
                        (_, b'w') => Some(Direction::West),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }
}

impl Day24 {
    fn get_floor_pattern(&self, input: &Vec<Vec<Direction>>) -> HashSet<(isize, isize)> {
        let mut turned: HashSet<(isize, isize)> = HashSet::new();

        for directions in input {
            let mut curr_x = 0;
            let mut curr_y = 0;
            for direction in directions {
                let (dx, dy) = direction.diff();
                curr_x += dx;
                curr_y += dy;
            }
            if turned.contains(&(curr_x, curr_y)) {
                turned.remove(&(curr_x, curr_y));
            } else {
                turned.insert((curr_x, curr_y));
            }
        }
        turned
    }
}
