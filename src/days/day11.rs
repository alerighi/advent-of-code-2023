use std::str::FromStr;

use crate::problem::AoCProblem;
use anyhow::{bail, Error, Result};

#[derive(Debug, Default)]
pub struct AoCDay11 {
    num_galaxies_y: Vec<usize>,
    num_galaxies_x: Vec<usize>,
    galaxies: Vec::<(usize, usize)>,
}

impl FromStr for AoCDay11 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut galaxy = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(match char {
                    '.' => false,
                    '#' => true,
                    _ => bail!("invalid input"),
                });
            }

            galaxy.push(row);
        }

        let mut num_galaxies_y = Vec::new();
        for row in &galaxy {
            num_galaxies_y.push(row.iter().filter(|&&e| e).count());
        }

        let mut num_galaxies_x = Vec::new();
        for col in 0..galaxy[0].len() {
            num_galaxies_x.push(galaxy.iter().filter(|r| r[col]).count());
        }

        let mut galaxies = Vec::<(usize, usize)>::new();
        for (y, line) in galaxy.iter().enumerate() {
            for (x, &galaxy) in line.iter().enumerate() {
                if galaxy {
                    galaxies.push((y, x));
                }
            }
        }

        Ok(Self { num_galaxies_y, num_galaxies_x, galaxies })
    }
}

impl AoCDay11 {

    fn dist(&self, &(y1, x1): &(usize, usize), &(y2, x2): &(usize, usize), multiplier: u64) -> u64 {
        if x1 > x2 {
            self.dist(&(y1, x2), &(y2, x1), multiplier)
        } else {
            let mut dist: u64 = 0;
            for x in x1..x2 {
                if self.num_galaxies_x[x] == 0 {
                    dist += multiplier;
                } else {
                    dist += 1;
                }
            }
            for y in y1..y2 {
                if self.num_galaxies_y[y] == 0 {
                    dist += multiplier;
                } else {
                    dist += 1;
                }
            }
            dist
        }
    }

    fn solve(&self, multiplier: u64) -> u64 {
        let mut result = 0;
        for (i, a) in self.galaxies.iter().enumerate() {
            for b in self.galaxies.iter().skip(i) {
                result += self.dist(a, b, multiplier);
            }
        }

        result
    }
}

impl AoCProblem for AoCDay11 {
    fn solve_part1(&self) -> Result<String> {
        Ok(self.solve(2).to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        Ok(self.solve(1_000_000).to_string())
    }
}
