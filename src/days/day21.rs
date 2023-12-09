use std::str::FromStr;

use crate::problem::AoCProblem;
use anyhow::{bail, Error, Result};

#[derive(Debug, Default)]
pub struct AoCDay21 {}

impl FromStr for AoCDay21 {
    type Err = Error;

    fn from_str(_: &str) -> Result<Self> {
        bail!("not implemented")
    }
}

impl AoCProblem for AoCDay21 {
    fn solve_part1(&self) -> Result<String> {
        bail!("not implemented")
    }

    fn solve_part2(&self) -> Result<String> {
        bail!("not implemented")
    }
}
