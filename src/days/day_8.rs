//! This is the day 8 module.
//! The instructions are available here: https://adventofcode.com/2021/day/8

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 8;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        todo!()
    }
}

fn part_1() {
    todo!()
}

fn part_2() {
    todo!()
}

fn identify_digit(pattern: &str) -> Option<usize> {
    match pattern.len() {
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

#[cfg(test)]
mod test_day {

    const TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce";

    #[test]
    #[ignore]
    fn test_part_1() {}

    #[test]
    #[ignore]
    fn test_part_2() {}
}
