use crate::utils::tokenizer::StringTokenizer;
use crate::{problem::AoCProblem, utils::tokenizer::Tokenizer};
use anyhow::Result;

#[derive(Debug)]
struct Game {
    own_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct AoCDay4 {
    games: Vec<Game>,
}

impl AoCProblem for AoCDay4 {
    fn parse_line(&mut self, line: String) -> Result<()> {
        let mut tokenizer = StringTokenizer::from(line.as_str());
        tokenizer.read_until(':');

        let mut own_numbers: Vec<u32> = Vec::new();
        while tokenizer.peek_char() != Some('|') {
            own_numbers.push(tokenizer.next_digit().parse()?);
            tokenizer.skip_whitespace();
        }

        let mut winning_numbers: Vec<u32> = Vec::new();
        while tokenizer.peek_char().is_some() {
            winning_numbers.push(tokenizer.next_digit().parse()?);
        }

        self.games.push(Game {
            winning_numbers,
            own_numbers,
        });

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut points: u32 = 0;

        for game in &self.games {
            let winning_games: u32 = game.own_numbers
                .iter()
                .map(|&n| {
                    if game.winning_numbers.iter().any(|&w| w == n) {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            if winning_games > 0 {
                points += 2_u32.pow(winning_games - 1);
            }
        }

        Ok(points.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut multiplier = vec![1usize; self.games.len()];
        for (i, game) in self.games.iter().enumerate() {
            let winning_games: usize = game.own_numbers
                .iter()
                .map(|&n| {
                    if game.winning_numbers.iter().any(|&w| w == n) {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            for j in 1..=winning_games {
                multiplier[i + j] += multiplier[i];
            }
        }

        Ok(multiplier.iter().sum::<usize>().to_string())
    }
}
