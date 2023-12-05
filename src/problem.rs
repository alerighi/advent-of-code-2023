use std::{fmt::Debug, io::BufRead};

use anyhow::Result;

pub trait AoCProblem: Debug {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()>;
    fn solve_part1(&self) -> Result<String>;
    fn solve_part2(&self) -> Result<String>;
}
