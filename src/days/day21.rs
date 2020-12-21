use crate::day::Day;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use std::iter::FromIterator;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Food {
    names: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_split = line.split(" (contains ").collect::<Vec<&str>>();
        let names = line_split[0].split(" ").map(|x| x.to_string()).collect();
        let allergens = line_split[1]
            .strip_suffix(')')
            .expect("Invalid input")
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        Ok(Self { names, allergens })
    }
}

pub struct Day21 {}

impl Day for Day21 {
    type InputElement = Food;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        // Find allergen -> foods
        let mut allergen_food_mapping: HashMap<String, HashSet<String>> = HashMap::new();
        for food in input {
            let new_map: HashSet<String> =
                HashSet::from_iter(food.names.iter().map(|x| x.to_string()));
            for allergen in &food.allergens {
                if allergen_food_mapping.contains_key(allergen) {
                    allergen_food_mapping
                        .get_mut(allergen)
                        .expect("Unreachable")
                        .retain(|x| new_map.contains(x));
                } else {
                    allergen_food_mapping.insert(allergen.clone(), new_map.clone());
                }
            }
        }

        // For part 1, we can assume that all foods that occur in any of the allergen mappings, it
        // contains _some_ allergen and should thus not be counted
        let allergen_foods = allergen_food_mapping
            .values()
            .fold(HashSet::new(), |a, x| a.union(x).cloned().collect());
        input
            .iter()
            .map(|x| x.names.clone())
            .flatten()
            .filter(|x| !allergen_foods.contains(x))
            .count()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        // Find allergen -> foods
        let mut allergen_food_mapping: HashMap<String, HashSet<String>> = HashMap::new();
        for food in input {
            let new_map: HashSet<String> =
                HashSet::from_iter(food.names.iter().map(|x| x.to_string()));
            for allergen in &food.allergens {
                if allergen_food_mapping.contains_key(allergen) {
                    allergen_food_mapping
                        .get_mut(allergen)
                        .expect("Unreachable")
                        .retain(|x| new_map.contains(x));
                } else {
                    allergen_food_mapping.insert(allergen.clone(), new_map.clone());
                }
            }
        }

        // Find the actual mapping
        let mut mapping: BTreeMap<String, String> = BTreeMap::new();
        let mut mapped_foods = HashSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for (allergen, foods) in &allergen_food_mapping {
                let unknown_foods: Vec<String> = foods.difference(&mapped_foods).map(|x| x.to_string()).collect();
                if unknown_foods.len() == 1 {
                    changed = true;
                    mapping.insert(allergen.clone(), unknown_foods[0].clone());
                    mapped_foods.insert(unknown_foods[0].clone());
                }
            }
        }
        println!("{}", mapping.values().join(","));
        0
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse().expect("Invalid input"))
            .collect()
    }
}
