use std::{collections::{HashMap, VecDeque}, io::BufRead};

use crate::problem::AoCProblem;
use anyhow::{anyhow, bail, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/days/day05.pest"]
struct Day05Parser;

trait Parse {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
struct Conversion {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl Parse for Conversion {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized,
    {
        let inner_nodes: Vec<Pair<'_, Rule>> = node.into_inner().collect();

        Ok(Conversion {
            destination_start: inner_nodes[0].as_str().parse()?,
            source_start: inner_nodes[1].as_str().parse()?,
            length: inner_nodes[2].as_str().parse()?,
        })
    }
}

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    conversions: Vec<Conversion>,
}

impl Parse for Mapping {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut inner_nodes: VecDeque<Pair<'_, Rule>> = node.into_inner().collect();

        let mut mapping = Mapping {
            from: inner_nodes
                .pop_front()
                .ok_or(anyhow!("invalid input"))?
                .as_str()
                .into(),
            to: inner_nodes
                .pop_front()
                .ok_or(anyhow!("invalid input"))?
                .as_str()
                .into(),
            conversions: Vec::new(),
        };

        for inner_node in inner_nodes {
            mapping.conversions.push(Conversion::parse(inner_node)?);
        }

        mapping
            .conversions
            .sort_by(|a, b| a.source_start.cmp(&b.source_start));

        Ok(mapping)
    }
}

impl Mapping {
    fn map_to(&self, source: u64) -> u64 {
        for conversion in &self.conversions {
            if conversion.source_start <= source
                && source < conversion.source_start + conversion.length
            {
                let offset = source - conversion.source_start;

                return conversion.destination_start + offset;
            }
        }

        source
    }

    fn ranges(&self) -> Vec<(u64, u64)> {
        let mut current = 0u64;
        let mut result = Vec::new();

        for conversion in &self.conversions {
            if conversion.source_start != current {
                result.push((current, conversion.source_start));
            }
            result.push((
                conversion.source_start,
                conversion.source_start + conversion.length,
            ));
            current = conversion.source_start + conversion.length;
        }

        result.push((current, u64::MAX));

        result
    }

    fn ranges_in(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();

        for (s1, e1) in self.ranges() {
            assert!(s1 < e1);

            // B
            // E
            // S
            // T
            // E
            // M
            // M
            // I
            // E
            if e1 > start && s1 < end {
                let s2 = s1.max(start);
                let e2 = e1.min(end);

                assert!(s2 < e2);

                result.push((s2, e2));
            }
        }

        result
    }
}

#[derive(Debug, Default)]
pub struct AoCDay5 {
    seeds: Vec<u64>,
    mapping: HashMap<String, Mapping>,
}

impl Parse for Vec<u64> {
    fn parse(node: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized,
    {
        node.into_inner().map(|n| n.as_str().parse::<u64>().map_err(anyhow::Error::from)).into_iter().collect::<Result<Vec<u64>>>()
    }
}

impl AoCDay5 {
    fn find_min(&self, category: &String, start: u64, end: u64) -> u64 {
        // eprintln!("{} {} {} ", category, start, end);
        assert!(start <= end, "{} {}", start, end);

        if let Some(mapping) = self.mapping.get(category) {
            let mut result = u64::MAX;
            for (a, b) in mapping.ranges_in(start, end) {
                let to_a = mapping.map_to(a);
                let to_b = mapping.map_to(b - 1);
                let mapped = self.find_min(&mapping.to, to_a, to_b + 1);
                result = result.min(mapped);
            }
            result
        } else {
            // no mapping left, we reached the bottom of the search
            start
        }
    }
}

impl AoCProblem for AoCDay5 {
    fn parse(&mut self, reader: &mut dyn BufRead) -> Result<()> {
        let mut content: String = String::new();
        reader.read_to_string(&mut content)?;
        let parsed = Day05Parser::parse(Rule::input, &content)?;
        for pair in parsed {
            match pair.as_rule() {
                Rule::seeds => {
                    self.seeds = Vec::<u64>::parse(pair)?;
                }
                Rule::map => {
                    let mapping = Mapping::parse(pair)?;
                    self.mapping.insert(mapping.from.clone(), mapping);
                }
                _ => {
                    bail!("parsing error");
                }
            }
        }

        Ok(())
    }

    fn solve_part1(&self) -> Result<String> {
        let mut result = u64::MAX;
        for &seed in &self.seeds {
            result = result.min(self.find_min(&"seed".into(), seed, seed + 1));
        }
        Ok(result.to_string())
    }

    fn solve_part2(&self) -> Result<String> {
        let mut result = u64::MAX;
        for i in 0..(self.seeds.len() / 2) {
            let start = self.seeds[i * 2];
            let length = self.seeds[i * 2 + 1];
            let end = start + length;

            result = result.min(self.find_min(&"seed".into(), start, end));
        }
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use super::{Conversion, Day05Parser, Mapping};

    #[test]
    fn parse_input() {
        let sample_input = include_str!("../../input/05/example.txt");
        let result = Day05Parser::parse(super::Rule::input, sample_input);

        assert!(result.is_ok());
    }

    #[test]
    fn test_mapping() {
        let mapping = Mapping {
            from: "a".into(),
            to: "b".into(),
            conversions: vec![Conversion {
                destination_start: 50,
                source_start: 98,
                length: 2,
            }],
        };

        assert_eq!(mapping.map_to(98), 50);
        assert_eq!(mapping.map_to(99), 51);
        assert_eq!(mapping.map_to(100), 100);
    }

    #[test]
    fn fills_void() {
        let mapping = Mapping {
            from: "a".into(),
            to: "b".into(),
            conversions: vec![
                Conversion {
                    destination_start: 50,
                    source_start: 1,
                    length: 2,
                },
                Conversion {
                    destination_start: 50,
                    source_start: 5,
                    length: 1,
                },
                Conversion {
                    destination_start: 50,
                    source_start: 6,
                    length: 2,
                },
            ],
        };

        let ranges = mapping.ranges();
        assert_eq!(ranges[0], (0, 1));
        assert_eq!(ranges[1], (1, 3));
        assert_eq!(ranges[2], (3, 5));
        assert_eq!(ranges[3], (5, 6));
        assert_eq!(ranges[4], (6, 8));
    }

    #[test]
    fn test_ranges_in() {
        let mapping = Mapping {
            from: "a".into(),
            to: "b".into(),
            conversions: vec![
                Conversion {
                    destination_start: 50,
                    source_start: 1,
                    length: 2,
                },
                Conversion {
                    destination_start: 50,
                    source_start: 5,
                    length: 1,
                },
                Conversion {
                    destination_start: 50,
                    source_start: 6,
                    length: 2,
                },
            ],
        };

        let ranges = mapping.ranges_in(3, 5);
        assert_eq!(ranges[0], (3, 5));
    }
}
