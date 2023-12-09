use std::{num::ParseIntError, str::FromStr};

use crate::problem::AoCProblem;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct AoCDay9 {
    values: Vec<Vec<i64>>,
}

fn get_next_value(values: &[i64]) -> i64 {
    let mut expansion: Vec<Vec<i64>> = vec![values.into(); 1];

    while !expansion.last().unwrap().iter().all(|&v| v == 0) {
        let mut row = Vec::new();
        let prev = expansion.last().unwrap();
        for j in 0..prev.len() - 1 {
            let v = prev[j + 1] - prev[j];
            row.push(v);
        }

        expansion.push(row);
    }

    expansion.iter().map(|v| *v.last().unwrap()).sum()
}

fn get_prev_value(values: &[i64]) -> i64 {
    let mut expansion: Vec<Vec<i64>> = vec![values.into(); 1];

    while !expansion.last().unwrap().iter().all(|&v| v == 0) {
        let mut row = Vec::new();
        let prev = expansion.last().unwrap();
        for j in 0..prev.len() - 1 {
            let v = prev[j + 1] - prev[j];
            row.push(v);
        }

        expansion.push(row);
    }

    expansion
        .iter()
        .rev()
        .map(|v| *v.first().unwrap())
        .fold(0, |acc, v| v - acc)
}

impl FromStr for AoCDay9 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            values: s
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|v| v.parse::<i64>())
                        .collect::<Result<Vec<i64>, ParseIntError>>()
                })
                .collect::<Result<Vec<Vec<i64>>, ParseIntError>>()?,
        })
    }
}

impl AoCProblem for AoCDay9 {
    fn solve_part1(&self) -> Result<String> {
        Ok(self
            .values
            .iter()
            .map(|v| get_next_value(v))
            .sum::<i64>()
            .to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        Ok(self
            .values
            .iter()
            .map(|v| get_prev_value(v))
            .sum::<i64>()
            .to_string())
    }
}
