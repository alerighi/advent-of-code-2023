mod days;
mod problem;
mod utils;
mod test_case;
mod args;

use clap::Parser;
use colored::{ColoredString, Colorize};

use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use problem::AoCProblem;

use anyhow::Result;

use crate::args::Args;
use crate::test_case::{TestCase, load_test_cases};
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


fn main() -> Result<()> {
    let args = Args::parse();

    println!("{} solving day {}", "==>".green().bold(), args.day);

    for case in load_test_cases(args.day)? {
        if let Some(filter) = &args.input_filter {
            if !case.name().contains(filter) {
                continue
            }
        }
        run_day(&args, &case);
    }

    Ok(())
}

fn run_day(args: &Args, case: &TestCase) {
    match args.day {
        1 => run_on_input::<AoCDay1>(args, case),
        2 => run_on_input::<AoCDay2>(args, case),
        3 => run_on_input::<AoCDay3>(args, case),
        4 => run_on_input::<AoCDay4>(args, case),
        5 => run_on_input::<AoCDay5>(args, case),
        6 => run_on_input::<AoCDay6>(args, case),
        7 => run_on_input::<AoCDay7>(args, case),
        8 => run_on_input::<AoCDay8>(args, case),
        9 => run_on_input::<AoCDay9>(args, case),
        10 => run_on_input::<AoCDay10>(args, case),
        11 => run_on_input::<AoCDay11>(args, case),
        12 => run_on_input::<AoCDay12>(args, case),
        13 => run_on_input::<AoCDay13>(args, case),
        14 => run_on_input::<AoCDay14>(args, case),
        15 => run_on_input::<AoCDay15>(args, case),
        16 => run_on_input::<AoCDay16>(args, case),
        17 => run_on_input::<AoCDay17>(args, case),
        18 => run_on_input::<AoCDay18>(args, case),
        19 => run_on_input::<AoCDay19>(args, case),
        20 => run_on_input::<AoCDay20>(args, case),
        21 => run_on_input::<AoCDay21>(args, case),
        22 => run_on_input::<AoCDay22>(args, case),
        23 => run_on_input::<AoCDay23>(args, case),
        24 => run_on_input::<AoCDay24>(args, case),
        25 => run_on_input::<AoCDay25>(args, case),
        _ => panic!("invalid day"),
    }
}

fn run_on_input<T>(args: &Args, case: &TestCase)
where
    T: AoCProblem,
    <T as FromStr>::Err: Display,
{
    println!(
        "{} Running on case: {}",
        "==>".green().bold(),
        case.name()
    );

    let parsing_time: Instant = Instant::now();
    let problem = case.input().parse::<T>();
    match problem {
        Ok(problem) => {
            println!(
                "  {} parsing OK ({}ms)",
                "->".blue().bold(),
                parsing_time.elapsed().as_millis()
            );

            if args.dump_input {
                println!("PARSED INPUT: {:#?}", problem);
            }

            let part_1 = solve_part(&problem, case, 1);
            let part_2 = solve_part(&problem, case, 2);

            if args.store_output {
                if let (Some(part_1), Some(part_2)) = (part_1, part_2) {
                    println!("  {} writing result to ans file", "->".blue().bold());
                    match case.write_output(&part_1, &part_2) {
                        Ok(_) => {}
                        Err(_) => println!("{} ", "==> write failed".red().bold()),
                    }
                }
            }
        }
        Err(error) => {
            println!("{} {}", "==>".red().bold(), error)
        }
    }
}

fn validate(ans: &String, expected: &Option<String>) -> ColoredString {
    if let Some(expected) = expected {
        if expected.trim() == ans {
            "CORRECT".green()
        } else {
            " WRONG ".red()
        }
    } else {
        "UNKNOWN".yellow()
    }
}

fn solve_part<T>(problem: &T, case: &TestCase, part: u32) -> Option<String>
where
    T: AoCProblem,
{
    let start_time = Instant::now();
    let solution = match part {
        1 => problem.solve_part1(),
        2 => problem.solve_part2(),
        _ => panic!("invalid part"),
    };
    let elapsed_time = start_time.elapsed().as_micros();

    let solution_string = solution
        .as_ref()
        .map(String::clone)
        .unwrap_or("ERROR".red().to_string());
    println!(
        "  {} part {} = {} [{}] ({}μs)",
        "->".blue().bold(),
        part,
        solution_string,
        validate(&solution_string, &case.output(part).map(String::from)),
        elapsed_time
    );

    solution.ok()
}
