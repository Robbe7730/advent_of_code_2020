use crate::day::Day;

use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Instruction {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Forward(isize),
    Left(isize),
    Right(isize),
}

impl Instruction {
    pub fn from_direction(direction: &Direction, amount: isize) -> Instruction {
        match direction {
            Direction::North => Instruction::North(amount),
            Direction::East  => Instruction::East(amount),
            Direction::South => Instruction::South(amount),
            Direction::West  => Instruction::West(amount),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl TryFrom<isize> for Direction {
    type Error = String;

    fn try_from(v: isize) -> Result<Self, Self::Error> {
        match v {
            x if x == 0 => Ok(Direction::North),
            x if x == 90 => Ok(Direction::East),
            x if x == 180 => Ok(Direction::South),
            x if x == 270 => Ok(Direction::West),
            x => Err(format!("Invalid angle: {}", x)),
        }
    }

}

impl Direction {
    pub fn rotate(start: Direction, amount: isize) -> Direction {
        Direction::try_from((start as isize + amount + 360) % 360)
            .expect("Invalid Direction")
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (letter, amount_str) = line.split_at(1);
        let amount = amount_str.parse().expect("Invalid Input");
        match letter.bytes().next().expect("Invalid Input") {
            b'N' => Ok(Instruction::North(amount)),
            b'E' => Ok(Instruction::East(amount)),
            b'S' => Ok(Instruction::South(amount)),
            b'W' => Ok(Instruction::West(amount)),
            b'F' => Ok(Instruction::Forward(amount)),
            b'L' => Ok(Instruction::Left(amount)),
            b'R' => Ok(Instruction::Right(amount)),
            _ => Err("Invalid Input".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ShipPosition {
    x: isize,
    y: isize,
    direction: Direction
}

impl ShipPosition {
    pub fn execute_step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::North(amount) => self.x += amount,
            Instruction::East(amount) => self.y += amount,
            Instruction::South(amount) => self.x -= amount,
            Instruction::West(amount) => self.y -= amount,
            Instruction::Forward(amount) => self.execute_step(
                &Instruction::from_direction(&self.direction, *amount)
            ),
            Instruction::Left(amount) => self.rotate(-amount),
            Instruction::Right(amount) => self.rotate(*amount),
        }
    }

    fn rotate(&mut self, amount: isize) {
        self.direction = Direction::rotate(self.direction, amount);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct WaypointShipPosition {
    waypoint_x: isize,
    waypoint_y: isize,
    ship_x: isize,
    ship_y: isize
}

impl WaypointShipPosition {
    pub fn execute_step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::North(amount) => self.waypoint_x += amount,
            Instruction::East(amount) => self.waypoint_y += amount,
            Instruction::South(amount) => self.waypoint_x -= amount,
            Instruction::West(amount) => self.waypoint_y -= amount,
            Instruction::Forward(amount) => { 
                self.ship_x += self.waypoint_x * amount;
                self.ship_y += self.waypoint_y * amount;
            }
            Instruction::Left(amount) => {
                for _ in 0..(amount / 90) {
                    let temp = self.waypoint_x;
                    self.waypoint_x = self.waypoint_y;
                    self.waypoint_y = -temp;
                }
            }
            Instruction::Right(amount) => {
                for _ in 0..(amount / 90) {
                    let temp = self.waypoint_x;
                    self.waypoint_x = -self.waypoint_y;
                    self.waypoint_y = temp;
                }
            }
        }
    }
}

pub struct Day12 {}

impl Day for Day12 {
    type InputElement = Instruction;
    type Output1 = isize;
    type Output2 = isize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let end_state = input.iter()
            .fold(ShipPosition {
                x: 0,
                y: 0,
                direction: Direction::East
            },|mut x,s| {x.execute_step(s); x});
        end_state.x.abs() + end_state.y.abs()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let end_state = input.iter()
            .fold(WaypointShipPosition {
                waypoint_x: 1,
                waypoint_y: 10,
                ship_x: 0,
                ship_y: 0,
            },|mut x,s| {x.execute_step(s); x});
        end_state.ship_x.abs() + end_state.ship_y.abs()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.lines().map(|x|
                        x.parse::<Self::InputElement>()
                        .expect("Invalid input")
                ).collect()
    }
}
