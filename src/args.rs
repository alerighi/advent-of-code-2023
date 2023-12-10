use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// day to solve
    pub day: u32,

    /// filter on the input file
    pub input_filter: Option<String>,

    /// dump the input
    #[arg(short, long)]
    pub dump_input: bool,

    /// store output on .ans file
    #[arg(short, long)]
    pub store_output: bool,
}
