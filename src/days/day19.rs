use crate::day::Day;

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Rule {
    Letter(char),
    Reference(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = line.chars().collect();
        if chars[0] == '"' {
            Ok(Rule::Letter(chars[1]))
        } else {
            Ok(Rule::Reference(
                line.split(" | ")
                    .map(|refs| {
                        refs.split(" ")
                            .map(|x| x.parse().expect("Invalid input"))
                            .collect()
                    })
                    .collect(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleCollection {
    rules: HashMap<usize, Rule>,
}

impl FromStr for RuleCollection {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rules: content
                .lines()
                .map(|line| {
                    let line_split: Vec<&str> = line.split(": ").collect();
                    let index: usize = line_split[0].parse().expect("Invalid input");
                    (index, line_split[1].parse().expect("Invalid input"))
                })
                .collect(),
        })
    }
}

impl RuleCollection {
    pub fn matches(&self, value: &str) -> bool {
        match self.match_len(value, 0, 0) {
            None => false,
            Some(x) => x >= value.len(),
        }
    }

    pub fn to_regex_part2(&self) -> Regex {
        let rule42 = self.to_regex_str(42);
        let rule31 = self.to_regex_str(31);
        let rule8 = format!("({})+", rule42);
        let mut rule11_parts = vec![];
        let mut curr_part = format!("({}{})", rule42, rule31);
        rule11_parts.push(curr_part.clone());
        // Research shows that this does not go over 3 deep
        for _ in 0..3 {
            curr_part = format!("({}{}{})", rule42, curr_part, rule31);
            rule11_parts.push(curr_part.clone());
        }
        let rule11 = rule11_parts.join("|");
        Regex::new(&format!("^({}({}))$", rule8, rule11)).expect("Invalid input")
    }

    fn to_regex_str(&self, value: usize) -> String {
        match self.rules.get(&value).expect("Invalid input") {
            Rule::Letter(x) => x.to_string(),
            Rule::Reference(x) => format!(
                "({})",
                x.iter()
                    .map(|chain| chain
                        .iter()
                        .map(|x| self.to_regex_str(*x))
                        .collect::<String>())
                    .join("|")
            ),
        }
    }

    fn match_len(&self, value: &str, char_idx: usize, rule_idx: usize) -> Option<usize> {
        //println!("Checking rule {} for pos {} : {:?}", rule_idx, char_idx, self.rules.get(&rule_idx));
        match self.rules.get(&rule_idx) {
            Some(Rule::Letter(x)) => {
                if value.chars().nth(char_idx) == Some(*x) {
                    Some(1)
                } else {
                    //println!("expected {} got {}", x, value.chars().nth(char_idx).expect("Invalid Input"));
                    None
                }
            }
            Some(Rule::Reference(v)) => {
                let mut ret = 0;
                for chain in v {
                    let mut curr_len = 0;
                    for rule in chain {
                        let match_found = self.match_len(value, char_idx + curr_len, *rule);
                        //println!("Back to {}", rule_idx);
                        if match_found == None {
                            //println!("No match :'(");
                            curr_len = 0;
                            break;
                        } else {
                            //println!("It's a match :D");
                            curr_len += match_found.unwrap();
                        }
                    }
                    //println!("-----");
                    if curr_len > ret {
                        //println!("New best match is {} long", curr_len);
                        ret = curr_len;
                    }
                }
                if ret == 0 {
                    None
                } else {
                    Some(ret)
                }
            }
            None => panic!("Invalid input"),
        }
    }
}

pub struct Day19 {}

impl Day for Day19 {
    type InputElement = (RuleCollection, Vec<String>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let (rules, text) = &input[0];
        text.iter()
            .filter(|t| rules.matches(t))
            //.inspect(|x| println!("{:#?}", x))
            .count()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let (rules, text) = &input[0];
        let regex = rules.to_regex_part2();
        // 389 -> too high
        // 386 -> too high
        // 374 -> incorrect
        text.iter()
            .filter(|t| regex.is_match(t))
            //.inspect(|x| println!("{:#?}", x))
            .count()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        let parts: Vec<&str> = content.split("\n\n").collect();
        let rules = parts[0].parse().expect("Invalid input");
        let inputs = parts[1].lines().map(|line| line.to_string()).collect();
        vec![(rules, inputs)]
    }
}
