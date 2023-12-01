use crate::problem::AoCProblem;

#[derive(Debug, Default)]
pub struct AoCDay1 {
    lines: Vec<String>,
}

const NUMBER_AS_LETTERS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl AoCProblem for AoCDay1 {
    fn parse_line(&mut self, line: String) {
        self.lines.push(line);
    }

    fn solve_part1(&self) -> String {
        let mut sum = 0u32;
        for line in &self.lines {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|v| v.to_digit(10).unwrap())
                .collect();

            if numbers.len() >= 1 {
                sum += (numbers.first().unwrap() * 10) + numbers.last().unwrap();
            }
        }
        sum.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut sum: usize = 0;
        for line in &self.lines {
            let mut digits: Vec<usize> = Vec::new();
            // no comment...
            for i in 0..line.len() {
                let sub = &line[i..];
                if let Some(c) = sub.chars().next() {
                    if c.is_digit(10) {
                        digits.push(c.to_digit(10).unwrap() as usize);
                    }
                }
                for (i, number) in NUMBER_AS_LETTERS.iter().enumerate() {
                    if sub.starts_with(number) {
                        digits.push(i);
                    }
                }
            }

            if digits.len() >= 1 {
                sum += (digits.first().unwrap() * 10) + digits.last().unwrap();
            }
        }
        sum.to_string()
    }
}
