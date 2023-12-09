use std::{fmt::Debug, str::FromStr};

use anyhow::Result;

pub trait AoCProblem: Debug + FromStr {
    fn solve_part1(&self) -> Result<String>;
    fn solve_part2(&self) -> Result<String>;
}
