use crate::day::Day;

use itertools::Itertools;

use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Life {
    active_fields: HashSet<(isize, isize, isize)>,
}

impl Life {
    pub fn neighbors(x: isize, y: isize, z: isize) -> Vec<(isize, isize, isize)> {
        (-1..=1)
            .map(|dx| {
                (-1..=1)
                    .map(move |dy| {
                        (-1..=1)
                            .filter(move |dz| dx != 0 || dy != 0 || *dz != 0)
                            .map(move |dz| (x + dx, y + dy, z + dz))
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }

    pub fn stay_alive(&self, x: isize, y: isize, z: isize) -> bool {
        if self.active_fields.contains(&(x, y, z)) {
            let c = Life::neighbors(x, y, z)
                .iter()
                .filter(|p| self.active_fields.contains(p))
                .count();
            c == 2 || c == 3
        } else {
            Life::neighbors(x, y, z)
                .iter()
                .filter(|p| self.active_fields.contains(p))
                .count()
                == 3
        }
    }

    pub fn to_4d(&self) -> Life4D {
        Life4D {
            active_fields: HashSet::from_iter(
                self.active_fields.iter().map(|(x, y, z)| (*x, *y, *z, 0)),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Life4D {
    active_fields: HashSet<(isize, isize, isize, isize)>,
}

impl Life4D {
    pub fn neighbors(x: isize, y: isize, z: isize, w: isize) -> Vec<(isize, isize, isize, isize)> {
        (-1..=1)
            .map(|dx| {
                (-1..=1)
                    .map(move |dy| {
                        (-1..=1)
                            .map(move |dz| {
                                (-1..=1)
                                    .filter(move |dw| dx != 0 || dy != 0 || dz != 0 || *dw != 0)
                                    .map(move |dw| (x + dx, y + dy, z + dz, w + dw))
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }

    pub fn stay_alive(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        if self.active_fields.contains(&(x, y, z, w)) {
            let c = Self::neighbors(x, y, z, w)
                .iter()
                .filter(|p| self.active_fields.contains(p))
                .count();
            c == 2 || c == 3
        } else {
            Self::neighbors(x, y, z, w)
                .iter()
                .filter(|p| self.active_fields.contains(p))
                .count()
                == 3
        }
    }
}

pub struct Day17 {}

impl Day for Day17 {
    type InputElement = Life;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input_vec: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut curr_state = input_vec[0].clone();
        for _ in 0..6 {
            let new_fields = HashSet::from_iter(
                curr_state
                    .active_fields
                    .iter()
                    .map(|(x, y, z)| Life::neighbors(*x, *y, *z))
                    .flatten()
                    .unique()
                    .filter(|(x, y, z)| curr_state.stay_alive(*x, *y, *z)),
            );
            curr_state = Life {
                active_fields: new_fields,
            };
        }
        curr_state.active_fields.len()
    }

    fn solve_part2(&self, input_vec: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut curr_state = input_vec[0].clone().to_4d();
        for _ in 0..6 {
            let new_fields = HashSet::from_iter(
                curr_state
                    .active_fields
                    .iter()
                    .map(|(x, y, z, w)| Life4D::neighbors(*x, *y, *z, *w))
                    .flatten()
                    .unique()
                    .filter(|(x, y, z, w)| curr_state.stay_alive(*x, *y, *z, *w)),
            );
            curr_state = Life4D {
                active_fields: new_fields,
            };
        }
        curr_state.active_fields.len()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        vec![Life {
            active_fields: HashSet::from_iter(
                content
                    .lines()
                    .enumerate()
                    .map(|(x, line)| {
                        line.bytes()
                            .enumerate()
                            .filter(|(_, val)| *val == b'#')
                            .map(move |(y, _)| (x as isize, y as isize, 0))
                    })
                    .flatten(),
            ),
        }]
    }
}
