use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use crate::problem::AoCProblem;
use anyhow::{anyhow, Result};
use sscanf::sscanf;

const START: &'static str = "AAA";
const DESTINATION: &'static str = "ZZZ";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction string"),
        }
    }
}

#[derive(Debug)]
struct Node(String, String);

impl Node {
    fn move_to(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.0.clone(),
            Direction::Right => self.1.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct AoCDay8 {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

// I admit, I used GPT for this... 
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

fn vector_lcm(numbers: Vec<usize>) -> usize {
    numbers.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}

impl AoCProblem for AoCDay8 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

        for c in lines[0].chars() {
            self.directions.push(Direction::from(c));
        }

        for line in &lines[2..] {
            let (from, left, right) = sscanf!(line, "{str} = ({str}, {str})").unwrap();
            self.nodes
                .insert(from.into(), Node(left.into(), right.into()));
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut current_node: String = START.into();
        let mut step: usize = 0;
        while current_node != DESTINATION {
            let direction = self.directions[step % self.directions.len()];
            let node = self
                .nodes
                .get(&current_node)
                .ok_or(anyhow!("node not found"))?;
            current_node = node.move_to(direction);
            step += 1;
        }

        Ok(step.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut current_nodes = self
            .nodes
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| k.clone())
            .collect::<Vec<String>>();
        let mut step: usize = 0;

        let mut terminal_states = vec![0usize; current_nodes.len()];

        while !terminal_states.iter().all(|&n| n != 0) {
            let direction = self.directions[step % self.directions.len()];
            current_nodes = current_nodes
                .into_iter()
                .map(|k| self.nodes.get(&k).unwrap().move_to(direction))
                .collect::<Vec<String>>();

            step += 1;

            for (i, node) in current_nodes.iter().enumerate() {
                if node.ends_with("Z") {
                    terminal_states[i] = step;
                }
            }
        }

        Ok(vector_lcm(terminal_states).to_string())
    }
}
