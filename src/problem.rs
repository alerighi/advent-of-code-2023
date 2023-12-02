use std::io::{self, Write};
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::fmt::Debug;

use anyhow::Result;

pub trait AoCProblem: Debug {
    fn parse_line(&mut self, line: String) -> anyhow::Result<()>;
    fn solve_part1(&self) -> Result<String>;
    fn solve_part2(&self) -> Result<String>;
}

pub fn create_template(day: u32) -> io::Result<()> {
    let p = format!("input/{:02}", day);
    let input_dir = Path::new(&p);
    if input_dir.exists() {
        return Ok(());
    }

    fs::create_dir_all(input_dir)?;
    File::create(input_dir.join("input.txt"))?;
    File::create(input_dir.join("example.txt"))?;

    let mut day_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("src/days/day{:02}.rs", day))?;
    writeln!(day_file,"use crate::problem::AoCProblem;\n")?;
    writeln!(day_file,"#[derive(Debug, Default)]")?;
    writeln!(day_file, "pub struct AoCDay{} {{\n}}", day)?;
    writeln!(day_file)?;
    writeln!(day_file, "impl AoCProblem for AoCDay{} {{", day)?;
    writeln!(day_file, "    fn parse_line(&mut self, line: String) {{")?;
    writeln!(day_file, "        // TODO")?;
    writeln!(day_file, "    }}")?;
    writeln!(day_file)?;
    writeln!(day_file, "    fn solve_part1(&self) -> String {{")?;
    writeln!(day_file, "        \"TODO\".into()")?;
    writeln!(day_file, "    }}")?;
    writeln!(day_file)?;
    writeln!(day_file, "    fn solve_part2(&self) -> String {{")?;
    writeln!(day_file, "        \"TODO\".into()")?;
    writeln!(day_file, "    }}")?;
    writeln!(day_file, "}}")?;

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/days/mod.rs")?;

    writeln!(mod_file, "pub mod day{:02};", day)?;

    Ok(())
}
