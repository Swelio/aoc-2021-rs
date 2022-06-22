#![deny(clippy::all)]

use clap::Parser;

use aoc_2021_rs::{days, DailySolution};

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
struct Cli {
    /// Day solution(s) to run with provided inputs in folder aoc_inputs.
    #[clap(short, long, min_values = 1, required = true)]
    running_day: Vec<u8>,
}

fn main() {
    let cli: Cli = Cli::parse();

    for requested_day_number in cli.running_day.into_iter() {
        match requested_day_number {
            days::day_1::Solution::DAY_NUMBER => days::day_1::Solution::run(),
            days::day_2::Solution::DAY_NUMBER => days::day_2::Solution::run(),
            days::day_3::Solution::DAY_NUMBER => days::day_3::Solution::run(),
            days::day_4::Solution::DAY_NUMBER => days::day_4::Solution::run(),
            days::day_5::Solution::DAY_NUMBER => days::day_5::Solution::run(),
            days::day_6::Solution::DAY_NUMBER => days::day_6::Solution::run(),
            days::day_7::Solution::DAY_NUMBER => days::day_7::Solution::run(),
            _ => panic!("the day {} is not implemented", requested_day_number),
        }
        .unwrap();
    }
}
