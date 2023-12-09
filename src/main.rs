mod days;
mod problem;
mod utils;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use problem::AoCProblem;

use anyhow::{anyhow, bail, Result};
use clap::Parser;

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

    /// dump the input
    #[arg(short, long)]
    dump_input: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("*** solving day {} ***", args.day);

    let dir = std::fs::read_dir(format!("input/{:02}", args.day))?;
    for file in dir {
        let path = file?.path();
        if !path.extension().is_some_and(|e| e == "ans") {
            let input = path.to_str().ok_or(anyhow!("parse failed"))?;

            match run_day(args.day, input, args.dump_input) {
                Ok(()) => {}
                Err(error) => println!("error running on input {}: {:?}", input, error),
            }
        }
    }

    Ok(())
}

fn run_day(day: u32, input: &str, dump_input: bool) -> Result<()> {
    match day {
        1 => run_on_input::<AoCDay1>(day, input, dump_input),
        2 => run_on_input::<AoCDay2>(day, input, dump_input),
        3 => run_on_input::<AoCDay3>(day, input, dump_input),
        4 => run_on_input::<AoCDay4>(day, input, dump_input),
        5 => run_on_input::<AoCDay5>(day, input, dump_input),
        6 => run_on_input::<AoCDay6>(day, input, dump_input),
        7 => run_on_input::<AoCDay7>(day, input, dump_input),
        8 => run_on_input::<AoCDay8>(day, input, dump_input),
        9 => run_on_input::<AoCDay9>(day, input, dump_input),
        10 => run_on_input::<AoCDay10>(day, input, dump_input),
        11 => run_on_input::<AoCDay11>(day, input, dump_input),
        12 => run_on_input::<AoCDay12>(day, input, dump_input),
        13 => run_on_input::<AoCDay13>(day, input, dump_input),
        14 => run_on_input::<AoCDay14>(day, input, dump_input),
        15 => run_on_input::<AoCDay15>(day, input, dump_input),
        16 => run_on_input::<AoCDay16>(day, input, dump_input),
        17 => run_on_input::<AoCDay17>(day, input, dump_input),
        18 => run_on_input::<AoCDay18>(day, input, dump_input),
        19 => run_on_input::<AoCDay19>(day, input, dump_input),
        20 => run_on_input::<AoCDay20>(day, input, dump_input),
        21 => run_on_input::<AoCDay21>(day, input, dump_input),
        22 => run_on_input::<AoCDay22>(day, input, dump_input),
        23 => run_on_input::<AoCDay23>(day, input, dump_input),
        24 => run_on_input::<AoCDay24>(day, input, dump_input),
        25 => run_on_input::<AoCDay25>(day, input, dump_input),
        _ => bail!("day not implemented"),
    }
}

fn run_on_input<T>(day: u32, input: &str, dump_input: bool) -> Result<()>
where
    T: AoCProblem,
    anyhow::Error: From<T::Err>,
{
    println!("Running on input: {}", input);

    let parsing_time: Instant = Instant::now();
    let input_string = fs::read_to_string(input)?;

    let problem = input_string.parse::<T>()?;

    println!(
        "parsing finished ({}ms)",
        parsing_time.elapsed().as_millis()
    );

    if dump_input {
        println!("PARSED INPUT: {:#?}", problem);
    }

    let mut ans_part_1 = String::new();
    let mut ans_part_2 = String::new();
    let ans_path = PathBuf::from(input).with_extension("ans");
    if ans_path.exists() {
        let ans_file = File::open(ans_path)?;
        let mut ans_reader = BufReader::new(ans_file);
        ans_reader.read_line(&mut ans_part_1)?;
        ans_reader.read_line(&mut ans_part_2)?;
    }

    let validate = |ans: &String, expected: &String| -> &str {
        if expected.is_empty() {
            "UNKNOWN"
        } else if expected.trim() == ans {
            "CORRECT"
        } else {
            " WRONG "
        }
    };

    let time_1 = Instant::now();
    let result_1 = problem.solve_part1().unwrap_or("ERROR".into());
    println!(
        "DAY{} PART 1 solution = {} [{}] ({}ms)",
        day,
        result_1,
        validate(&result_1, &ans_part_1),
        time_1.elapsed().as_millis()
    );

    let time_2 = Instant::now();
    let result_2 = problem.solve_part2().unwrap_or("ERROR".into());
    println!(
        "DAY{} PART 2 solution = {} [{}] ({}ms)",
        day,
        result_2,
        validate(&result_2, &ans_part_2),
        time_2.elapsed().as_millis()
    );

    Ok(())
}
