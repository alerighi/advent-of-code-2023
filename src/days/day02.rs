use std::str::FromStr;

use crate::problem::AoCProblem;

use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
struct Color(u32, u32, u32);

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

#[derive(Debug, Default)]
pub struct AoCDay2 {
    games: Vec<Vec<Color>>,
}

impl FromStr for AoCDay2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut result = AoCDay2::default();
        for line in s.lines() {
            let mut games: Vec<Color> = Vec::new();
            for game in line
                .split(':')
                .nth(1)
                .ok_or(anyhow!("invalid input"))?
                .split(';')
            {
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;
                for color_line in game.split(',') {
                    let parts = color_line.trim().split(' ').collect::<Vec<&str>>();
                    let number = parts[0].parse::<u32>()?;
                    let color = parts[1];

                    match color {
                        "red" => {
                            r = number;
                        }
                        "green" => {
                            g = number;
                        }
                        "blue" => {
                            b = number;
                        }
                        _ => {
                            bail!("invalid input");
                        }
                    }
                }
                games.push(Color(r, g, b));
            }

            result.games.push(games);
        }

        Ok(result)
    }
}

impl AoCProblem for AoCDay2 {
    fn solve_part1(&self) -> Result<String> {
        let mut result: usize = 0;
        for (id, rounds) in self.games.iter().enumerate() {
            if rounds.iter().all(|&Color(r, g, b)| r <= R && g <= G && b <= B) {
                result += id + 1;
            }
        }

        Ok(result.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut result: u32 = 0;
        for game in &self.games {
            let (mut max_r, mut max_g, mut max_b) = (0u32, 0u32, 0u32);
            for &Color(r, g, b) in game {
                max_r = max_r.max(r);
                max_b = max_b.max(b);
                max_g = max_g.max(g);
            }
            result += max_r * max_b * max_g;
        }

        Ok(result.to_string())
    }
}
