use crate::day::Day;

use std::collections::HashMap;

use core::num::ParseIntError;
use core::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Write(usize, usize), // address, value
    Mask(usize, usize),  // 0-mask, 1-mask
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_split: Vec<&str> = line.split(" = ").collect();
        if line_split[0] == "mask" {
            let mut zero_mask = 0;
            let mut one_mask = 0;
            for letter in line_split[1].bytes() {
                zero_mask <<= 1;
                one_mask <<= 1;
                match letter {
                    b'0' => zero_mask |= 1,
                    b'1' => one_mask |= 1,
                    b'X' => (),
                    _ => panic!("Invalid input"),
                }
            }
            Ok(Instruction::Mask(zero_mask, one_mask))
        } else {
            let n = line_split[0].len();
            let addr = &line_split[0][4..n - 1];
            Ok(Instruction::Write(addr.parse()?, line_split[1].parse()?))
        }
    }
}

struct ProgramState {
    zero_mask: usize,
    one_mask: usize,
    memory: HashMap<usize, usize>,
}

impl ProgramState {
    pub fn new() -> Self {
        ProgramState {
            zero_mask: 0,
            one_mask: 0,
            memory: HashMap::new(),
        }
    }

    pub fn execute_instruction_1(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Mask(zero_mask, one_mask) => {
                self.zero_mask = *zero_mask;
                self.one_mask = *one_mask;
            }
            Instruction::Write(addr, value) => {
                self.memory
                    .insert(*addr, (value | self.one_mask) & !self.zero_mask);
            }
        }
    }

    pub fn execute_instruction_2(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Mask(zero_mask, one_mask) => {
                self.zero_mask = *zero_mask;
                self.one_mask = *one_mask;
            }
            Instruction::Write(addr, value) => {
                self.possible_addresses(*addr).iter().for_each(|a| {
                    self.memory.insert(*a, *value);
                })
            }
        }
    }

    fn possible_addresses(&self, old_addr: usize) -> Vec<usize> {
        let mut floating_map = !(self.one_mask | self.zero_mask) & 0xfffffffff;
        let base_addr = (old_addr | self.one_mask) & !floating_map;

        let mut ret = vec![base_addr];

        let mut i = 0;

        while floating_map != 0 {
            if (floating_map & 1) == 1 {
                for x in ret.clone() {
                    ret.push(x | (1 << i));
                }
            }
            floating_map >>= 1;
            i += 1;
        }

        ret
    }
}

pub struct Day14 {}

impl Day for Day14 {
    type InputElement = Instruction;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut program_state = ProgramState::new();
        input
            .iter()
            .for_each(|x| program_state.execute_instruction_1(x));
        program_state.memory.values().sum()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let mut program_state = ProgramState::new();
        input
            .iter()
            .for_each(|x| program_state.execute_instruction_2(x));
        program_state.memory.values().sum()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Instruction>().expect("Invalid input"))
            .collect()
    }
}
