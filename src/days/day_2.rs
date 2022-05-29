//! This is the day 2 module.
//! The instructions are available here: https://adventofcode.com/2021/day/2

use std::fs;
use std::path::Path;

use regex::RegexBuilder;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 2;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let input_content = fs::read_to_string(input_path)?;

        let part_1_result = part_1(&input_content);
        println!("Final position score: {}", part_1_result);

        let part_2_result = part_2(&input_content);
        println!("Final aimed position score: {}", part_2_result);

        Ok(())
    }
}

fn part_1(input_content: &str) -> usize {
    let mut vertical_pos = 0;
    let mut horizontal_pos = 0;

    let operation_regex = RegexBuilder::new(r"^(?P<operator>forward|down|up) (?P<units>\d+)$")
        .multi_line(true)
        .build()
        .unwrap();

    for cap in operation_regex.captures_iter(input_content) {
        let units = cap["units"].parse::<isize>().unwrap();
        match &cap["operator"] {
            "forward" => horizontal_pos += units,
            "down" => vertical_pos += units,
            "up" => vertical_pos -= units,
            _ => panic!("unknown operator '{}'", &cap["operator"]),
        }
    }

    usize::try_from(vertical_pos * horizontal_pos).unwrap()
}
fn part_2(input_content: &str) -> usize {
    let mut aim = 0;
    let mut vertical_pos = 0;
    let mut horizontal_pos = 0;

    let operation_regex = RegexBuilder::new(r"^(?P<operator>forward|down|up) (?P<units>\d+)$")
        .multi_line(true)
        .build()
        .unwrap();

    for cap in operation_regex.captures_iter(input_content) {
        let units = cap["units"].parse::<isize>().unwrap();
        match &cap["operator"] {
            "forward" => {
                horizontal_pos += units;
                vertical_pos += aim * units;
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => panic!("unknown operator '{}'", &cap["operator"]),
        }
    }

    usize::try_from(vertical_pos * horizontal_pos).unwrap()
}

#[cfg(test)]
mod test_day {
    use super::{part_1, part_2};

    /// Sample lines from guidelines
    const TEST_LINES: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_LINES), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_LINES), 900);
    }
}
