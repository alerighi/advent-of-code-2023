use std::{io::BufRead, num::ParseIntError};

use crate::problem::AoCProblem;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct AoCDay9 {
    values: Vec<Vec<i64>>,
}

fn get_next_value(values: &Vec<i64>) -> i64 {
    let mut expansion: Vec<Vec<i64>> = vec![values.clone(); 1];

    while !expansion.last().unwrap().iter().all(|&v| v == 0) {
        let mut row = Vec::new();
        let prev = expansion.last().unwrap();
        for j in 0..prev.len() - 1 {
            let v = prev[j + 1] - prev[j];
            row.push(v);
        }

        expansion.push(row);
    }

    expansion.iter().map(|v| v.last().unwrap().clone()).sum()
}

fn get_prev_value(values: &Vec<i64>) -> i64 {
    let mut expansion: Vec<Vec<i64>> = vec![values.clone(); 1];

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
        .map(|v| v.first().unwrap().clone())
        .fold(0, |acc, v| v - acc)
}

impl AoCProblem for AoCDay9 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        for line in reader.lines() {
            self.values.push(
                line?
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<i64>())
                    .collect::<Result<Vec<i64>, ParseIntError>>()?,
            )
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        Ok(self
            .values
            .iter()
            .map(get_next_value)
            .sum::<i64>()
            .to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        Ok(self
            .values
            .iter()
            .map(get_prev_value)
            .sum::<i64>()
            .to_string())
    }
}
