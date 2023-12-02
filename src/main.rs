mod days;
mod problem;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use problem::AoCProblem;

use clap::Parser;
use anyhow::{anyhow, Result};

use crate::days::day01::AoCDay1;
use crate::days::day02::AoCDay2;
use crate::days::day03::AoCDay3;
use crate::days::day04::AoCDay4;
use crate::days::day05::AoCDay5;
use crate::days::day06::AoCDay6;
use crate::days::day07::AoCDay7;
use crate::days::day08::AoCDay8;
use crate::days::day09::AoCDay9;
use crate::days::day10::AoCDay10;
use crate::days::day11::AoCDay11;
use crate::days::day12::AoCDay12;
use crate::days::day13::AoCDay13;
use crate::days::day14::AoCDay14;
use crate::days::day15::AoCDay15;
use crate::days::day16::AoCDay16;
use crate::days::day17::AoCDay17;
use crate::days::day18::AoCDay18;
use crate::days::day19::AoCDay19;
use crate::days::day20::AoCDay20;
use crate::days::day21::AoCDay21;
use crate::days::day22::AoCDay22;
use crate::days::day23::AoCDay23;
use crate::days::day24::AoCDay24;
use crate::days::day25::AoCDay25;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day to solve
    day: u32,

    /// generate template for the day
    #[arg(short, long)]
    template: bool,

    /// dump the input
    #[arg(short, long)]
    dump_input: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.template {
        problem::create_template(args.day)?;
    }

    println!("*** solving day {} ***", args.day);

    let dir = std::fs::read_dir(format!("input/{:02}", args.day))?;

    for file in dir {
        let path = file?.path();
        let input = path.to_str().ok_or(anyhow!("parse failed"))?;

        match run_on_input(args.day, input, args.dump_input) {
            Ok(()) => {},
            Err(error) => println!("error running on input {}: {:?}", input, error),
        }
    }

    Ok(())
}

fn run_on_input(day: u32, input: &str, dump_input: bool) -> Result<()> {
    let mut problem: Box<dyn AoCProblem> = match day {
        1 => Box::<AoCDay1>::default(),
        2 => Box::<AoCDay2>::default(),
        3 => Box::<AoCDay3>::default(),
        4 => Box::<AoCDay4>::default(),
        5 => Box::<AoCDay5>::default(),
        6 => Box::<AoCDay6>::default(),
        7 => Box::<AoCDay7>::default(),
        8 => Box::<AoCDay8>::default(),
        9 => Box::<AoCDay9>::default(),
        10 => Box::<AoCDay10>::default(),
        11 => Box::<AoCDay11>::default(),
        12 => Box::<AoCDay12>::default(),
        13 => Box::<AoCDay13>::default(),
        14 => Box::<AoCDay14>::default(),
        15 => Box::<AoCDay15>::default(),
        16 => Box::<AoCDay16>::default(),
        17 => Box::<AoCDay17>::default(),
        18 => Box::<AoCDay18>::default(),
        19 => Box::<AoCDay19>::default(),
        20 => Box::<AoCDay20>::default(),
        21 => Box::<AoCDay21>::default(),
        22 => Box::<AoCDay22>::default(),
        23 => Box::<AoCDay23>::default(),
        24 => Box::<AoCDay24>::default(),
        25 => Box::<AoCDay25>::default(),
        _ => panic!("day not yet implemented"),
    };

    println!("Running on input: {}", input);

    let file = File::open(input)?;
    let buffer_reader = BufReader::new(file);
    for line in buffer_reader.lines() {
        match line {
            Ok(line) => problem.parse_line(line)?,
            Err(_) => break,
        }
    }

    if dump_input {
        println!("PARSED INPUT: {:#?}", problem);
    }

    let time_1 = Instant::now();
    let result_1 = problem.solve_part1();
    println!(
        "DAY{} PART 1 solution = {} ({}ms)",
        day,
        result_1.unwrap_or("ERROR".into()),
        time_1.elapsed().as_millis()
    );

    let time_2 = Instant::now();
    let result_2 = problem.solve_part2();
    println!(
        "DAY{} PART 2 solution = {} ({}ms)",
        day,
        result_2.unwrap_or("ERROR".into()),
        time_2.elapsed().as_millis()
    );

    Ok(())
}
