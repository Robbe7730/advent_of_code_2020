use crate::day::Day;

use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn contains(&self, x: usize) -> bool {
        x >= self.start && x <= self.end
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldRule {
    name: String,
    ranges: Vec<Range>
}

impl FieldRule {
    pub fn contains(&self, x: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(x))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    rules: Vec<FieldRule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>
}

pub struct Day16 {}

impl Day for Day16 {
    type InputElement = Input;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input_vec: &Vec<Self::InputElement>) -> Self::Output1 {
        let input = input_vec[0].clone();
        let ranges = input.rules.iter().map(|x| x.ranges.clone()).flatten();
        let mut ret = 0;

        for ticket in input.nearby_tickets {
            for field in ticket {
                if !(ranges.clone().any(|x| x.contains(field))) {
                    ret += field;
                }
            }
        }
        ret
    }

    fn solve_part2(&self, input_vec: &Vec<Self::InputElement>) -> Self::Output2 {
        let input = input_vec[0].clone();
        let ranges = input.rules.iter().map(|x| x.ranges.clone()).flatten();
        let valid_tickets = input.nearby_tickets.iter()
            .filter(|ticket|
                    ticket.iter()
                    .all(|x|
                         ranges.clone()
                         .any(|r| r.contains(*x))
                        )
                   );
        let mut options: Vec<HashSet<&FieldRule>> = vec![HashSet::from_iter(input.rules.iter());input.my_ticket.len()];

        for (i, &value) in input.my_ticket.iter().enumerate() {
            options[i].retain(|&x| x.contains(value));
        }
        for ticket in valid_tickets {
            for (i, &value) in ticket.iter().enumerate() {
                options[i].retain(|&x| x.contains(value));
            }
        }

        let mut final_mapping: Vec<Option<&&FieldRule>> = vec![None; input.my_ticket.len()];
        let mut changed = true;
        while changed {
            changed = false;
            for (i, option) in options.iter().enumerate() {
                let mut actual_option = option.iter().filter(|x| !final_mapping.contains(&Some(x)));
                let first = actual_option.next();
                let second = actual_option.next();
                if first != None && second == None {
                    changed = true;
                    final_mapping[i] = first;
                }
            }
        }

        final_mapping.iter().zip(input.my_ticket.iter())
                            .filter(|(m,_)| m.is_some() && m.unwrap().name.starts_with("departure"))
                            .map(|(_,x)| x)
                            .product()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        let sections: Vec<&str> = content.split("\n\n").collect();
        
        let rules = sections[0].lines().map(|line| {
            let line_split: Vec<&str> = line.split(": ").collect();
            let ranges = line_split[1].split(" or ").map(|x| {
                let range_split: Vec<&str> = x.split("-").collect();
                Range {
                    start: range_split[0].parse().expect("Invalid input"),
                    end: range_split[1].parse().expect("invalid input")
                }
            }).collect();
            FieldRule {
                name: line_split[0].to_string(),
                ranges: ranges
            }
        }).collect();

        let my_ticket = sections[1].lines().nth(1).expect("Invalid input").split(',').map(|x| x.parse().expect("Invalid input")).collect();

        let nearby_tickets = sections[2].lines().skip(1).map(|x| {
            x.split(',').map(|x| x.parse().expect("Invalid input")).collect()
        }).collect();
        
        vec![Input {
            rules: rules,
            my_ticket: my_ticket,
            nearby_tickets: nearby_tickets
        }]
    }
}
