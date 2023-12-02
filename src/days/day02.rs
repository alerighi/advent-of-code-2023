use crate::problem::AoCProblem;

use anyhow::{Result, anyhow, bail};

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14; 

#[derive(Debug, Default)]
pub struct AoCDay2 {
    games: Vec<Vec<(u32, u32, u32)>>,
}

impl AoCProblem for AoCDay2 {
    fn parse_line(&mut self, line: String) -> Result<()> {
        let mut games: Vec<(u32, u32, u32)> = Vec::new();
        for game in line.split(':').nth(1).ok_or(anyhow!("invalid input"))?.split(';') {
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
            games.push((r, g, b));
        }

        self.games.push(games);

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut result: usize = 0;
        for (id, rounds) in self.games.iter().enumerate() {
            if rounds.iter().all(|&(r, g, b)| r <= R && g <= G && b <= B) {
                result += id + 1;
            }
        }

        Ok(result.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut result: u32 = 0;
        for game in &self.games {
            let (mut max_r, mut max_g, mut max_b) = (0u32, 0u32, 0u32);
            for &(r, g, b) in game {
                max_r = max_r.max(r);
                max_b = max_b.max(b);
                max_g = max_g.max(g);
            }
            result += max_r * max_b * max_g;
        }

        Ok(result.to_string())
    }
}
