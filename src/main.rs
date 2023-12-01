mod days;
mod problem;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use problem::AoCProblem;

use clap::Parser;

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

fn main() {
    let args = Args::parse();

    if args.template {
        problem::create_template(args.day).unwrap();
        return;
    }

    println!("*** solving day {} ***", args.day);

    let dir = std::fs::read_dir(format!("input/{:02}", args.day)).unwrap();

    for file in dir {
        let path = file.unwrap().path();

        run_on_input(args.day, path.to_str().unwrap(), args.dump_input);
    }
}

fn run_on_input(day: u32, input: &str, dump_input: bool) {
    let mut problem: Box<dyn AoCProblem> = match day {
        1 => Box::new(days::day01::AoCDay1::default()),
        2 => Box::new(days::day02::AoCDay2::default()),
        3 => Box::new(days::day03::AoCDay3::default()),
        4 => Box::new(days::day04::AoCDay4::default()),
        5 => Box::new(days::day05::AoCDay5::default()),
        6 => Box::new(days::day06::AoCDay6::default()),
        7 => Box::new(days::day07::AoCDay7::default()),
        8 => Box::new(days::day08::AoCDay8::default()),
        9 => Box::new(days::day09::AoCDay9::default()),
        10 => Box::new(days::day10::AoCDay10::default()),
        11 => Box::new(days::day11::AoCDay11::default()),
        12 => Box::new(days::day12::AoCDay12::default()),
        13 => Box::new(days::day13::AoCDay13::default()),
        14 => Box::new(days::day14::AoCDay14::default()),
        15 => Box::new(days::day15::AoCDay15::default()),
        16 => Box::new(days::day16::AoCDay16::default()),
        17 => Box::new(days::day17::AoCDay17::default()),
        18 => Box::new(days::day18::AoCDay18::default()),
        19 => Box::new(days::day19::AoCDay19::default()),
        20 => Box::new(days::day20::AoCDay20::default()),
        21 => Box::new(days::day21::AoCDay21::default()),
        22 => Box::new(days::day22::AoCDay22::default()),
        23 => Box::new(days::day23::AoCDay23::default()),
        24 => Box::new(days::day24::AoCDay24::default()),
        25 => Box::new(days::day25::AoCDay25::default()),
        _ => panic!("day not yet implemented"),
    };

    println!("Running on input: {}", input);

    let file = File::open(input).expect("error opening file");
    let buffer_reader = BufReader::new(file);
    for line in buffer_reader.lines() {
        match line {
            Ok(line) => problem.parse_line(line),
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
        result_1,
        time_1.elapsed().as_millis()
    );

    let time_2 = Instant::now();
    let result_2 = problem.solve_part2();
    println!(
        "DAY{} PART 2 solution = {} ({}ms)",
        day,
        result_2,
        time_2.elapsed().as_millis()
    );
}
