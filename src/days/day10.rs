use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::problem::AoCProblem;
use anyhow::{bail, Error, Result};
use colored::{ColoredString, Colorize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Pipe,
    Loop,
}

#[derive(Debug, Default)]
pub struct AoCDay10 {
    map: Vec<Vec<Cell>>,
    edges: HashMap<Point, HashSet<Point>>,
    cycle_length: u32,
    input: Vec<Vec<char>>,
}

impl FromStr for AoCDay10 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut start = Point::default();
        let mut edges: HashSet<(Point, Point)> = HashSet::new();
        let mut map: Vec<Vec<Cell>> = Vec::new();
        let mut lines: Vec<Vec<char>> = Vec::new();

        for (y, line) in input.lines().enumerate() {
            lines.push(line.chars().collect());
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let y = y as i32;
                let x = x as i32;
                let p = Point(y, x);

                match c {
                    '.' => {}
                    'S' => {
                        start = p;
                        edges.insert((start, Point(y - 1, x)));
                        edges.insert((start, Point(y + 1, x)));
                        edges.insert((start, Point(y, x + 1)));
                        edges.insert((start, Point(y, x - 1)));
                    }
                    '-' => {
                        edges.insert((p, Point(y, x - 1)));
                        edges.insert((p, Point(y, x + 1)));
                    }
                    '|' => {
                        edges.insert((p, Point(y + 1, x)));
                        edges.insert((p, Point(y - 1, x)));
                    }
                    'F' => {
                        edges.insert((p, Point(y + 1, x)));
                        edges.insert((p, Point(y, x + 1)));
                    }
                    'L' => {
                        edges.insert((p, Point(y - 1, x)));
                        edges.insert((p, Point(y, x + 1)));
                    }
                    'J' => {
                        edges.insert((p, Point(y - 1, x)));
                        edges.insert((p, Point(y, x - 1)));
                    }
                    '7' => {
                        edges.insert((p, Point(y, x - 1)));
                        edges.insert((p, Point(y + 1, x)));
                    }
                    _ => bail!("invalid input"),
                }

                row.push(if c == '.' { Cell::Empty } else { Cell::Pipe });
            }

            map.push(row);
        }

        let mut edges_map: HashMap<Point, HashSet<Point>> = HashMap::new();
        for &(a, b) in edges.iter() {
            if edges.contains(&(b, a)) {
                // a -> b
                if let Some(v) = edges_map.get_mut(&a) {
                    v.insert(b);
                } else {
                    let mut set = HashSet::new();
                    set.insert(b);
                    edges_map.insert(a, set);
                }
                // b -> a
                if let Some(v) = edges_map.get_mut(&b) {
                    v.insert(a);
                } else {
                    let mut set = HashSet::new();
                    set.insert(a);
                    edges_map.insert(b, set);
                }
            }
        }

        let mut problem = Self {
            edges: edges_map,
            map,
            cycle_length: 0,
            input: lines,
        };

        problem.cycle_length = problem.cycle_length(start) / 2;

        Ok(problem)
    }
}

impl AoCDay10 {
    fn cycle_length(&mut self, p: Point) -> u32 {
        self.map[p.0 as usize][p.1 as usize] = Cell::Loop;
        if let Some(next) = self.edges.get(&p) {
            let next = next
                .iter()
                .filter(|n| self.map[n.0 as usize][n.1 as usize] == Cell::Pipe)
                .next();
            if let Some(&next) = next {
                1 + self.cycle_length(next)
            } else {
                1
            }
        } else {
            0
        }
    }

    fn explore(&self, p: Point, visited: &mut HashSet<Point>) {
        for p in [
            Point(p.0 - 1, p.1),
            Point(p.0 + 1, p.1),
            Point(p.0, p.1 - 1),
            Point(p.0, p.1 + 1),
        ] {
            let Point(y, x) = p;
            if x >= 0
                && y >= 0
                && y < self.map.len() as i32
                && x < self.map[y as usize].len() as i32
            {
                let c = self.map[y as usize][x as usize];
                if c != Cell::Pipe {
                    if !visited.contains(&p) {
                        visited.insert(p);
                        self.explore(p, visited);
                    }
                }
            }
        }
    }
}

impl AoCProblem for AoCDay10 {
    fn solve_part1(&self) -> Result<String> {
        Ok(self.cycle_length.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut count = 0;
        for (y, row) in self.map.iter().enumerate() {
            let mut in_loop = false;
            let mut prev_curve = ' ';
            for (x, &cell) in row.iter().enumerate() {
                let c = self.input[y as usize][x as usize];
                if cell == Cell::Loop {
                    if (prev_curve == 'L' && c == '7')
                        || (prev_curve == 'F' && c == 'J')
                        || c == '|'
                        || c == 'S'
                    {
                        in_loop = !in_loop;
                        prev_curve = ' ';
                    }
                    if (prev_curve == 'L' && c == 'J') || (prev_curve == 'F' && c == '7') {
                        prev_curve = ' ';
                    }
                    if c == 'L' || c == 'F' {
                        prev_curve = c;
                    }
                } else if (cell == Cell::Empty || cell == Cell::Pipe) && in_loop {
                    count += 1;
                }
            }
            assert_eq!(prev_curve, ' ');
        }

        Ok(count.to_string())
    }
}
