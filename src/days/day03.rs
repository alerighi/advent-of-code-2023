use std::collections::HashSet;

use crate::problem::AoCProblem;
use anyhow::Result;

#[derive(Debug, Copy, Clone)]
enum GridCell {
    Empty,
    Symbol(char),
    Number(u32, u32),
}

#[derive(Debug, Default)]
pub struct AoCDay3 {
    id: u32,
    grid: Vec<Vec<GridCell>>,
}

impl AoCDay3 {
    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i < self.grid.len() && j < self.grid[i].len()
    }

    fn near_cells(&self, i: usize, j: usize) -> Vec<GridCell> {
        let mut result: Vec<GridCell> = Vec::new();

        for (y, x) in [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ] {
            if self.in_bounds(y, x) {
                result.push(self.grid[y][x]);
            }
        }

        result
    }
}

impl AoCProblem for AoCDay3 {
    fn parse_line(&mut self, line: String) -> Result<()> {
        let mut row: Vec<GridCell> = Vec::new();
        let mut number: u32 = 0;
        let mut i: u32 = 0;

        for c in line.chars() {
            if c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap();
                i += 1;
            } else {
                for _ in 0..i {
                    row.push(GridCell::Number(self.id, number));
                }
                self.id += 1;
                number = 0;
                i = 0;

                if c == '.' {
                    row.push(GridCell::Empty);
                } else {
                    row.push(GridCell::Symbol(c));
                }
            }
        }

        for _ in 0..i {
            row.push(GridCell::Number(self.id, number));
        }

        self.grid.push(row);

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut result: u32 = 0;
        let mut already_sum = HashSet::new();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if let GridCell::Symbol(_) = self.grid[i][j] {
                    for c in self.near_cells(i, j) {
                        if let GridCell::Number(id, n) = c {
                            if !already_sum.contains(&id) {
                                already_sum.insert(id);
                                result += n;
                            }
                        }
                    }
                }
            }
        }

        Ok(result.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut result: u32 = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if let GridCell::Symbol('*') = self.grid[i][j] {
                    let mut nears: HashSet<(u32, u32)> = HashSet::new();
                    for c in self.near_cells(i, j) {
                        if let GridCell::Number(id, n) = c {
                            nears.insert((id, n));
                        }
                    }
                    if nears.len() == 2 {
                        result += nears.iter().map(|&(_, x)| x).product::<u32>();
                    }
                }
            }
        }

        Ok(result.to_string())
    }
}
