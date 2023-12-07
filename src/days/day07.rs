use std::{cmp::Ordering, collections::HashMap, io::BufRead};

use crate::problem::AoCProblem;
use anyhow::{bail, Result};

#[derive(Debug, Eq, Ord, Clone)]
struct Hand(Vec<char>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Game(Hand, u32);

#[derive(Debug, Default)]
pub struct AoCDay7 {
    games: Vec<Game>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn score_of(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap(),
        _ => unreachable!(),
    }
}

fn score_of_with_j(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap(),
        'J' => 1,
        _ => unreachable!(),
    }
}

impl Score {
    // UGLY!!!! didn't want to refactor all of this for part 2...
    fn with_j(hand: &Hand) -> Score {
        let mut count = HashMap::<char, u32>::new();
        for &c in &hand.0 {
            count.insert(c, 1 + count.get(&c).unwrap_or(&0));
        }

        if let Some((_, c)) = count.iter().filter(|(&c, _)| c != 'J').map(|(&c, &n)| (n, c)).max() {
            Score::from(&Hand(hand.0.iter().map(|&e| if e == 'J' { c } else { e }).collect::<Vec<char>>()))
        } else {
            Score::from(hand)
        }
    }
}

impl From<&Hand> for Score {
    fn from(hand: &Hand) -> Score {
        let mut count = HashMap::<char, u32>::new();
        for &c in &hand.0 {
            count.insert(c, 1 + count.get(&c).unwrap_or(&0));
        }

        let mut counts = count.values().cloned().collect::<Vec<u32>>();
        counts.sort();
        match counts.len() {
            1 => Score::FiveOfAKind,
            2 => {
                if counts[0] == 1 {
                    Score::FourOfAKind
                } else {
                    Score::FullHouse
                }
            }
            3 => {
                if counts[2] == 3 {
                    Score::ThreeOfAKind
                } else {
                    Score::TwoPair
                }
            }
            4 => Score::OnePair,
            _ => Score::HighCard,
        }
    }
}

fn cmp(a: &Hand, b: &Hand) -> Ordering {
    let score_a = Score::from(a);
    let score_b = Score::from(b);

    let compared = score_a.cmp(&score_b);
    if compared == Ordering::Equal {
        for (&c_a, &c_b) in a.0.iter().zip(b.0.iter()) {
            let v_a = score_of(c_a);
            let v_b = score_of(c_b);
            let o = v_a.cmp(&v_b);
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    } else {
        compared
    }
}

fn cmp_j(a: &Hand, b: &Hand) -> Ordering {
    let score_a = Score::with_j(a);
    let score_b = Score::with_j(b);

    let compared = score_a.cmp(&score_b);
    if compared == Ordering::Equal {
        for (&c_a, &c_b) in a.0.iter().zip(b.0.iter()) {
            let v_a = score_of_with_j(c_a);
            let v_b = score_of_with_j(c_b);
            let o = v_a.cmp(&v_b);
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    } else {
        compared
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        cmp(self, other) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp(self, other))
    }
}

impl AoCProblem for AoCDay7 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        for line in reader.lines() {
            if let Some((cards, score)) = line?.split_once(' ') {
                self.games
                    .push(Game(Hand(cards.chars().collect()), score.parse()?));
            }
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut game_with_rank = self.games.clone();
        game_with_rank.sort();

        Ok(game_with_rank
            .iter()
            .enumerate()
            .map(|(rank, Game(_, bet))| bet * (rank as u32 + 1))
            .sum::<u32>()
            .to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut game_with_rank = self.games.clone();
        game_with_rank.sort_by(|a, b| cmp_j(&a.0, &b.0));

        Ok(game_with_rank
            .iter()
            .enumerate()
            .map(|(rank, Game(_, bet))| bet * (rank as u32 + 1))
            .sum::<u32>()
            .to_string())    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmp() {
        assert!(Score::FiveOfAKind > Score::FourOfAKind);
    }

    #[test]
    fn compute_score() {
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'A', 'A', 'A'])),
            Score::FiveOfAKind
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'A', 'A', 'B'])),
            Score::FourOfAKind
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'A', 'B', 'B'])),
            Score::FullHouse
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'A', 'B', 'C'])),
            Score::ThreeOfAKind
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'B', 'B', 'C'])),
            Score::TwoPair
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'A', 'B', 'C', 'D'])),
            Score::OnePair
        );
        assert_eq!(
            Score::from(&Hand(vec!['A', 'B', 'C', 'D', 'E'])),
            Score::HighCard
        );
    }

    #[test]
    fn hand_ordering() {
        assert!(Hand(vec!['A', 'A', 'A', 'A', 'A']) > Hand(vec!['A', 'A', 'A', 'A', 'B']));
        assert!(Hand(vec!['2', 'B', 'C', 'D', 'E']) < Hand(vec!['A', 'B', 'C', 'D', 'E']));
        assert!(Hand(vec!['A', 'A', 'A', 'A', 'A']) == Hand(vec!['A', 'A', 'A', 'A', 'A']));
        assert!(Hand(vec!['A', 'A', 'A', 'A', 'K']) < Hand(vec!['A', 'A', 'A', 'A', 'A']));
    }
}
