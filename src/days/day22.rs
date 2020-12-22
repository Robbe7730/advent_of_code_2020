use crate::day::Day;

use std::collections::HashSet;
use std::collections::VecDeque;

use std::str::FromStr;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Player {
    cards: VecDeque<usize>,
}

impl FromStr for Player {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(Player {
            cards: content
                .lines()
                .skip(1)
                .map(|x| x.parse().expect("Invalid Input"))
                .collect(),
        })
    }
}

pub struct Day22 {}

impl Day for Day22 {
    type InputElement = Player;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut player_one = input[0].clone();
        let mut player_two = input[1].clone();
        while !(player_one.cards.is_empty() || player_two.cards.is_empty()) {
            let first_card = player_one.cards.pop_front().unwrap();
            let second_card = player_two.cards.pop_front().unwrap();
            if first_card > second_card {
                player_one.cards.push_back(first_card);
                player_one.cards.push_back(second_card);
            } else {
                player_two.cards.push_back(second_card);
                player_two.cards.push_back(first_card);
            }
        }
        let mut ret = 0;
        if player_one.cards.is_empty() {
            for (index, card) in player_two.cards.iter().rev().enumerate() {
                ret += (index + 1) * *card
            }
        } else {
            for (index, card) in player_one.cards.iter().rev().enumerate() {
                ret += (index + 1) * *card
            }
        }
        ret
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        let (_, winning_player, _) = self.play_recursive(input[0].clone(), input[1].clone());
        let mut ret = 0;
        for (index, card) in winning_player.cards.iter().rev().enumerate() {
            ret += (index + 1) * *card
        }
        ret
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .split("\n\n")
            .map(|x| x.parse().expect("Invalid input"))
            .collect()
    }
}

impl Day22 {
    fn play_recursive(
        &self,
        mut player_one: Player,
        mut player_two: Player,
    ) -> (usize, Player, Player) {
        // Keep track of seen states
        let mut seen_states: HashSet<(Player, Player)> = HashSet::new();

        while !(player_one.cards.is_empty() || player_two.cards.is_empty())
            && !seen_states.contains(&(player_one.clone(), player_two.clone()))
        {
            // Observe state
            seen_states.insert((player_one.clone(), player_two.clone()));

            // Draw cards
            let first_card = player_one.cards.pop_front().unwrap();
            let second_card = player_two.cards.pop_front().unwrap();

            // Determine winner
            let winner;
            if first_card <= player_one.cards.iter().count()
                && second_card <= player_two.cards.iter().count()
            {
                // Recurse
                let mut new_player_one = player_one.clone();
                let mut new_player_two = player_two.clone();

                new_player_one.cards.resize(first_card, 0);
                new_player_two.cards.resize(second_card, 0);

                //println!("Recurse");
                let result = self.play_recursive(new_player_one, new_player_two);
                winner = result.0;
            } else {
                // Regular play
                if first_card > second_card {
                    winner = 0;
                } else {
                    winner = 1;
                }
            }

            //println!("Player {} wins this round", winner+1);
            // Put cards back
            if winner == 0 {
                player_one.cards.push_back(first_card);
                player_one.cards.push_back(second_card);
            } else {
                player_two.cards.push_back(second_card);
                player_two.cards.push_back(first_card);
            }
        }
        //println!("Step out");
        if !player_one.cards.is_empty() {
            (0, player_one, player_two)
        } else {
            (1, player_two, player_one)
        }
    }
}
