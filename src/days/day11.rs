use crate::day::Day;

use itertools::iproduct;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum SeatType {
    Floor,
    Empty,
    Taken,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SeatingRow {
    seats: Vec<SeatType>,
}

impl FromStr for SeatingRow {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(SeatingRow {
            seats: line.bytes()
                        .map(|x| match x {
                            b'.' => SeatType::Floor,
                            b'L' => SeatType::Empty,
                            b'#' => SeatType::Taken,
                            _ => panic!("Invalid symbol")
                        })
                        .collect()
            })
    }
}

pub struct Day11 {}

impl Day for Day11 {
    type InputElement = SeatingRow;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut curr_state;
        let mut new_state = input.to_vec();
        let num_rows = input.len() as isize;
        let num_cols = input[0].seats.len() as isize;
        let mut changed = true;
        while changed {
            changed = false;
            curr_state = new_state.clone();
            for (i, row) in curr_state.iter().enumerate() {
                for (j, seat_type) in row.seats.iter().enumerate() {
                    let neighbours_taken: usize = iproduct!(-1..=1, -1..=1)
                                            .map(|(dx, dy)| ((i as isize)+dx, (j as isize)+dy))
                                            .filter(|(x, y)| 
                                                        *x >= 0 &&
                                                        *y >= 0 && 
                                                        *x < num_rows && 
                                                        *y < num_cols)
                                            .map(|(x, y)| (curr_state[x as usize].seats[y as usize] == SeatType::Taken) as usize)
                                            .sum();
                    new_state[i].seats[j] = match seat_type {
                        SeatType::Floor => SeatType::Floor,
                        SeatType::Taken if neighbours_taken < 5 => SeatType::Taken,
                        SeatType::Empty if neighbours_taken == 0 => {changed=true; SeatType::Taken},
                        SeatType::Taken => {changed=true; SeatType::Empty}
                        SeatType::Empty => SeatType::Empty
                    }
                }
            }
        }
        new_state.iter()
                    .map(|x| x.seats.iter()
                                    .map(|x| (*x == SeatType::Taken) as usize)
                                    .sum::<usize>()
                    ).sum()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut curr_state;
        let mut new_state = input.to_vec();
        let mut changed = true;
        let num_rows = input.len();
        let num_cols = input[0].seats.len();
        while changed {
            changed = false;
            curr_state = new_state.clone();
            // Caluclate visible neighbours
            let mut visible_n  = vec![vec![false; num_cols]; num_rows];
            let mut visible_ne = vec![vec![false; num_cols]; num_rows];
            let mut visible_w  = vec![vec![false; num_cols]; num_rows];
            let mut visible_nw = vec![vec![false; num_cols]; num_rows];

            for (x, y) in iproduct!(0..num_rows, 0..num_cols) {
                visible_w[x][y]  = y > 0 && match curr_state[x].seats[y-1] {
                    SeatType::Taken => true,
                    SeatType::Empty => false,
                    SeatType::Floor => visible_w[x][y-1],
                };
                visible_nw[x][y]  = y > 0 && x > 0 && match curr_state[x-1].seats[y-1] {
                    SeatType::Taken => true,
                    SeatType::Empty => false,
                    SeatType::Floor => visible_nw[x-1][y-1],
                };
                visible_n[x][y]  = x > 0 && match curr_state[x-1].seats[y] {
                    SeatType::Taken => true,
                    SeatType::Empty => false,
                    SeatType::Floor => visible_n[x-1][y],
                };
                visible_ne[x][y]  = x > 0 && y < num_cols-1 && match curr_state[x-1].seats[y+1] {
                    SeatType::Taken => true,
                    SeatType::Empty => false,
                    SeatType::Floor => visible_ne[x-1][y+1],
                };
            }

            let mut visible_e  = vec![vec![false; num_cols]; num_rows];
            let mut visible_se = vec![vec![false; num_cols]; num_rows];
            let mut visible_s  = vec![vec![false; num_cols]; num_rows];
            let mut visible_sw = vec![vec![false; num_cols]; num_rows];

            for (x, y) in iproduct!((0..num_rows).rev(), (0..num_cols).rev()) {
                    visible_e[x][y]  = y < num_cols-1 && match curr_state[x].seats[y+1] {
                        SeatType::Taken => true,
                        SeatType::Empty => false,
                        SeatType::Floor => visible_e[x][y+1],
                    };
                    visible_se[x][y] = y < num_cols-1 && x < num_rows-1 && match curr_state[x+1].seats[y+1] {
                        SeatType::Taken => true,
                        SeatType::Empty => false,
                        SeatType::Floor => visible_se[x+1][y+1],
                    };
                    visible_s[x][y]  = x < num_rows-1 && match curr_state[x+1].seats[y] {
                        SeatType::Taken => true,
                        SeatType::Empty => false,
                        SeatType::Floor => visible_s[x+1][y],
                    };
                    visible_sw[x][y] = x < num_rows-1 && y > 0 && match curr_state[x+1].seats[y-1] {
                        SeatType::Taken => true,
                        SeatType::Empty => false,
                        SeatType::Floor => visible_sw[x+1][y-1],
                    };
            }

            // Generate new state
            for (x, y) in iproduct!(0..num_rows, 0..num_cols) {
                let num_visible = {
                    visible_n[x][y] as usize +
                    visible_ne[x][y] as usize +
                    visible_e[x][y] as usize +
                    visible_se[x][y] as usize +
                    visible_s[x][y] as usize +
                    visible_sw[x][y] as usize +
                    visible_w[x][y] as usize +
                    visible_nw[x][y] as usize
                };
                new_state[x].seats[y] = match curr_state[x].seats[y] {
                    SeatType::Floor => SeatType::Floor,
                    SeatType::Taken if num_visible < 5 => SeatType::Taken,
                    SeatType::Empty if num_visible == 0 => {changed=true; SeatType::Taken},
                    SeatType::Taken => {changed=true; SeatType::Empty},
                    SeatType::Empty => SeatType::Empty
                }
            }
        }
        new_state.iter()
                    .map(|x| x.seats.iter()
                                    .map(|x| (*x == SeatType::Taken) as usize)
                                    .sum::<usize>()
                    ).sum()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.lines().map(|x|
                        x.parse::<Self::InputElement>()
                        .expect("Invalid input")
                ).collect()
    }
}
