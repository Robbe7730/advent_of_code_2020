use crate::day::Day;

use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AOCOperationType {
    NOP,
    ACC,
    JMP
}

impl FromStr for AOCOperationType {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line.trim() {
            "nop" => Ok(AOCOperationType::NOP),
            "acc" => Ok(AOCOperationType::ACC),
            "jmp" => Ok(AOCOperationType::JMP),
            _ => Err("No such instruction".to_string())
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AOCOperation {
    operation_type: AOCOperationType,
    argument: isize,
}

impl FromStr for AOCOperation {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_split: Vec<&str> = line.split(" ").collect();
        Ok(AOCOperation {
            operation_type: line_split[0].parse()?,
            argument: line_split[1].parse().map_err(|x:ParseIntError| x.to_string())?
        })
    }
}

impl AOCState {
    pub fn execute(&mut self, operation: &AOCOperation) {
        match operation.operation_type {
            AOCOperationType::NOP => self.ip += 1,
            AOCOperationType::ACC => {
                self.accumulator += operation.argument;
                self.ip += 1;
            }
            AOCOperationType::JMP => self.ip += operation.argument
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AOCState {
    accumulator: isize,
    ip: isize,
}

impl AOCState {
    pub fn run(input: &Vec<AOCOperation>) -> AOCState {
        let mut state = AOCState { 
            accumulator: 0,
            ip: 0
        };
        let mut visited = HashSet::new();
        loop {
            if state.ip < 0 || state.ip >= (input.len() as isize) {
                break;
            }
            state.execute(&input[state.ip as usize]);
            if !visited.insert(state.ip) {
                break;
            }
        }
        state
    }
}

pub struct Day08 {}

struct NopJmpChange {
    input: Vec<AOCOperation>,
    last_changed: usize,
}

impl Iterator for NopJmpChange {
    type Item = Vec<AOCOperation>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, op)) = self.input
                                       .iter()
                                       .enumerate()
                                       .skip(self.last_changed)
                                       .filter(|(_, x)|
                                               x.operation_type == AOCOperationType::JMP ||
                                               x.operation_type == AOCOperationType::NOP
                                       ).next() {
            let mut ret = self.input.clone();
            let new_type = match op.operation_type {
                AOCOperationType::JMP => AOCOperationType::NOP,
                AOCOperationType::NOP => AOCOperationType::JMP,
                _ => panic!("Unreachable statement"),
            };
            self.last_changed = index+1;
            ret[index] = AOCOperation {
                operation_type: new_type,
                argument: op.argument,
            };
            Some(ret)
        } else {
            self.last_changed = self.input.len();
            None
        }
    }
}

impl NopJmpChange {
    pub fn new(input: &Vec<AOCOperation>) -> NopJmpChange {
        NopJmpChange {
            input: input.clone().to_vec(),
            last_changed: 0,
        }
    }
}

impl Day for Day08 {
    type InputElement = AOCOperation;
    type Output1 = isize;
    type Output2 = isize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        AOCState::run(input).accumulator
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let nop_jmp_change = NopJmpChange::new(input);
        let input_len = input.len() as isize;
        nop_jmp_change.map(|input| AOCState::run(&input))
                      .filter(|state| state.ip == input_len)
                      .next()
                      .expect("No result")
                      .accumulator
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.lines().map(|x|
                        x.parse::<Self::InputElement>()
                        .expect("Invalid input")
                ).collect()
    }
}
