use crate::day::Day;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    modifier: String,
    color: String,
    contains: Vec<ContainingRule>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContainingRule {
    modifier: String,
    color: String,
    amount: usize,
}

impl FromStr for Rule {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let line_split: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        let mut contains = vec![];
        if line_split[4] != "no" {
            let mut i = 4;
            while i < line_split.len() {
                contains.push(ContainingRule {
                    amount: line_split[i].parse()?,
                    modifier: line_split[i + 1].to_string(),
                    color: line_split[i + 2].to_string(),
                });
                i += 4
            }
        }
        Ok(Rule {
            modifier: line_split[0].to_string(),
            color: line_split[1].to_string(),
            contains: contains,
        })
    }
}

impl Rule {
    pub fn can_hold(&self, color: &String, modifier: &String) -> bool {
        self.contains
            .iter()
            .filter(|x| x.color == color.to_string() && x.modifier == modifier.to_string())
            .next()
            != None
    }

    pub fn containing_bags(&self, all_rules: &Vec<Rule>) -> usize {
        self.contains
            .iter()
            .map(|x| {
                x.amount
                    * all_rules
                        .iter()
                        .filter(|r| r.color == x.color && r.modifier == x.modifier)
                        .next()
                        .expect("No such bag")
                        .containing_bags(all_rules)
            })
            .sum::<usize>()
            + 1
    }
}

pub struct Day07 {}

impl Day for Day07 {
    type InputElement = Rule;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(("shiny".to_string(), "gold".to_string()));
        queue.push_back(("shiny".to_string(), "gold".to_string()));
        while !queue.is_empty() {
            let (modifier, color) = queue.pop_front().expect("Empty queue");
            visited.insert((modifier.to_string(), color.to_string()));
            input
                .iter()
                .filter(|x| {
                    !visited.contains(&(x.modifier.to_string(), x.color.to_string()))
                        && x.can_hold(&color, &modifier)
                })
                .for_each(|x| queue.push_back((x.modifier.to_string(), x.color.to_string())));
        }
        visited.len() - 1
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input
            .iter()
            .filter(|x| x.color == "gold" && x.modifier == "shiny")
            .next()
            .expect("No shiny gold bag")
            .containing_bags(input)
            - 1
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Rule>().expect("Invalid input"))
            .collect()
    }
}
