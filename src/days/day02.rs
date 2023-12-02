use crate::problem::AoCProblem;

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14; 

#[derive(Debug, Default)]
pub struct AoCDay2 {
    games: Vec<Vec<(u32, u32, u32)>>,
}

impl AoCProblem for AoCDay2 {
    fn parse_line(&mut self, line: String) {
        let mut games: Vec<(u32, u32, u32)> = Vec::new();
        for game in line.split(":").skip(1).next().unwrap().split(";") {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            for color_line in game.split(",") {
                let parts = color_line.trim().split(" ").collect::<Vec<&str>>();
                let number = parts[0].parse::<u32>().unwrap();
                let color = parts[1];

                match color {
                    "red" => {
                        r = number;
                    }
                    "green" => {
                        g = number;
                    }
                    "blue" => {
                        b = number;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            games.push((r, g, b));
        }

        self.games.push(games);
    }

    fn solve_part1(&self) -> String {
        let mut result: usize = 0;
        for (id, rounds) in self.games.iter().enumerate() {
            if rounds.iter().all(|&(r, g, b)| r <= R && g <= G && b <= B) {
                result += id + 1;
            }
        }

        result.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut result: u32 = 0;
        for game in &self.games {
            let (mut max_r, mut max_g, mut max_b) = (0u32, 0u32, 0u32);
            for &(r, g, b) in game {
                max_r = max_r.max(r);
                max_b = max_b.max(b);
                max_g = max_g.max(g);
            }
            result += max_r * max_b * max_g;
        }

        result.to_string()
    }
}
