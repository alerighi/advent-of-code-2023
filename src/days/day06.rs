use std::io::BufRead;

use crate::problem::AoCProblem;
use anyhow::{bail, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/days/day06.pest"]
struct Day06Parser;

trait Parse {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized;
}

impl Parse for Vec<u32> {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized,
    {
        node.into_inner()
            .map(|n| n.as_str().parse::<u32>().map_err(anyhow::Error::from))
            .into_iter()
            .collect::<Result<Vec<u32>>>()
    }
}

#[derive(Debug, Default)]
pub struct AoCDay6 {
    time: Vec<u32>,
    distance: Vec<u32>,
}

// binary search for the index where the distance starts to be greater than d
fn find_inversion_point(t: u64, d: u64, a: u64, b: u64) -> u64 {
    if a == b {
        a
    } else {
        let mid = (a + b) / 2;
        if mid * (t - mid) > d {
            find_inversion_point(t, d, a, mid)
        } else {
            find_inversion_point(t, d, mid + 1, b)
        }
    }
}

fn solve(t: u64, d: u64) -> u64 {
    let p = find_inversion_point(t, d, 0, t / 2);

    // num of trials is t + 1 minus the first index it takes to start winning (* 2 since the range are symmetric)
    t + 1 - p * 2
}

impl AoCProblem for AoCDay6 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let parsed = Day06Parser::parse(Rule::input, &content)?;
        for pair in parsed {
            match pair.as_rule() {
                Rule::time => {
                    self.time = Vec::<u32>::parse(pair)?;
                }
                Rule::distance => {
                    self.distance = Vec::<u32>::parse(pair)?;
                }
                _ => {
                    bail!("parsing error");
                }
            }
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        Ok(self
            .time
            .iter()
            .zip(self.distance.iter())
            .map(|(&time, &record_distance)| solve(time as u64, record_distance as u64))
            .product::<u64>()
            .to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let real_time: u64 = self
            .time
            .iter()
            .map(u32::to_string)
            .fold(String::new(), |acc, f| acc + &f)
            .parse::<u64>()?;
        let real_distance = self
            .distance
            .iter()
            .map(u32::to_string)
            .fold(String::new(), |acc, f| acc + &f)
            .parse::<u64>()?;

        Ok(solve(real_time, real_distance).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_inversion_point() {
        assert_eq!(find_inversion_point(7, 9, 0, 3), 2);
    }
}
