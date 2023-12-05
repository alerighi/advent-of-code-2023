use std::io::BufRead;

use crate::problem::AoCProblem;
use anyhow::{Result, anyhow};

#[derive(Debug, Default)]
pub struct AoCDay1 {
    lines: Vec<String>,
}

const NUMBER_AS_LETTERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl AoCProblem for AoCDay1 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        for line in reader.lines() {
            self.lines.push(line?);
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut sum = 0u32;
        for line in &self.lines {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .filter_map(|v| v.to_digit(10))
                .collect();

            if !numbers.is_empty() {
                sum += (numbers.first().ok_or(anyhow!("no solution"))? * 10) + numbers.last().ok_or(anyhow!("no solution"))?;
            }
        }
        
        Ok(sum.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut sum: usize = 0;
        for line in &self.lines {
            let mut digits: Vec<usize> = Vec::new();
            // no comment...
            for i in 0..line.len() {
                let sub = &line[i..];
                if let Some(c) = sub.chars().next() {
                    if c.is_ascii_digit() {
                        digits.push(c.to_digit(10).ok_or(anyhow!("invalid digit"))? as usize);
                    }
                }
                for (i, number) in NUMBER_AS_LETTERS.iter().enumerate() {
                    if sub.starts_with(number) {
                        digits.push(i);
                    }
                }
            }

            if !digits.is_empty() {
                sum += (digits.first().ok_or(anyhow!("no solution"))? * 10) + digits.last().ok_or(anyhow!("no solution"))?;
            }
        }
        
        Ok(sum.to_string())
    }
}
