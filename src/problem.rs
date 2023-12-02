use std::fmt::Debug;

use anyhow::Result;

pub trait AoCProblem: Debug {
    fn parse_line(&mut self, line: String) -> anyhow::Result<()>;
    fn solve_part1(&self) -> Result<String>;
    fn solve_part2(&self) -> Result<String>;
}
