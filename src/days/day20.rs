use crate::day::Day;

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
    edges: HashSet<Vec<bool>>,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = content.lines();
        let first = lines_iter.next().expect("Invalid input");
        Ok(Self::new(
            first
                .split(" ")
                .nth(1)
                .expect("Invalid input")
                .strip_suffix(":")
                .expect("Invalid input")
                .parse()
                .expect("Invalid input"),
            lines_iter
                .map(|x| x.bytes().map(|p| p == b'#').collect())
                .collect(),
        ))
    }
}

impl Tile {
    pub fn new(id: usize, pixels: Vec<Vec<bool>>) -> Self {
        let mut edges = HashSet::new();
        edges.insert(pixels[0].clone());
        edges.insert(pixels.last().expect("Invalid input").clone());
        edges.insert(pixels.iter().map(|x| x[0]).collect());
        edges.insert(
            pixels
                .iter()
                .map(|x| *x.last().expect("Invalid input"))
                .collect(),
        );
        edges.insert(pixels[0].iter().rev().cloned().collect());
        edges.insert(
            pixels
                .last()
                .expect("Invalid input")
                .iter()
                .rev()
                .cloned()
                .collect(),
        );
        edges.insert(pixels.iter().map(|x| x[0]).rev().collect());
        edges.insert(
            pixels
                .iter()
                .map(|x| *x.last().expect("Invalid input"))
                .rev()
                .collect(),
        );
        Self { id, pixels, edges }
    }

    pub fn fits(&self, other: &Tile) -> usize {
        let count = self.edges.intersection(&other.edges).count();
        count
    }
}

pub struct Day20 {}

impl Day for Day20 {
    type InputElement = Tile;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut ret = 1;
        for first in input {
            let mut num = 0;
            for second in input {
                let count = first.fits(second);
                if count > 0 {
                    num += 1;
                }
            }
            if num == 3 {
                ret *= first.id;
            }
        }
        ret
    }

    fn solve_part2(&self, _input: &Vec<Self::InputElement>) -> Self::Output2 {
        0
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .split("\n\n")
            .map(|x| x.parse().expect("Invalid input"))
            .collect()
    }
}
