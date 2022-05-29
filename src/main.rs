#![deny(clippy::all)]

use clap::Parser;

use aoc_2021_rs::{day_1, DailySolution};

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
struct Cli {
    /// Day solution(s) to run with provided inputs in folder aoc_inputs.
    #[clap(short, long, min_values = 1, required = true)]
    running_day: Vec<u8>,
}

fn main() {
    let cli = Cli::parse();

    for requested_day_number in cli.running_day.iter() {
        match requested_day_number {
            1 => day_1::Solution::run(),
            _ => panic!("the day {} is not implemented", requested_day_number),
        }
        .unwrap();
    }
}
