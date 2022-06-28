//! This is the day 8 module.
//! The instructions are available here: https://adventofcode.com/2021/day/8

use std::collections::{HashMap, HashSet};
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
        let lines = fs::read_to_string(input_path)?;

        let part_1_result = part_1(&lines);
        println!(
            "There are {} digits with unique number of segments",
            part_1_result
        );
        todo!()
    }
}

fn part_1(lines: &str) -> usize {
    lines
        .lines()
        .into_iter()
        .map(|line| {
            let output_side = line.split('|').nth(1).unwrap();
            output_side
                .split(' ')
                .into_iter()
                .filter_map(identify_digit)
                .count()
        })
        .sum()
}

fn part_2(lines: &str) -> usize {
    todo!()
}

fn identify_digit(pattern: &str) -> Option<usize> {
    match pattern.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

/// Find the best wires map. A wire map is an array of 7 positions.
/// For each position, there is a letter.
/// Positions are indexed from top, then left to right to the bottom
/// The following example is indexed as `[a, b, c, d, e, f, g]`:
/// ```
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
/// ```
fn decode_line(line: &str) -> String {
    let mut known_digits = HashMap::new();

    line.split(' ')
        .filter(|word| !word.is_empty() && *word != "|")
        .for_each(|word| match identify_digit(word) {
            None => {}
            Some(digit) => {
                if !known_digits.contains_key(&digit) {
                    let mut set = HashSet::new();
                    word.chars().for_each(|c| {
                        set.insert(c);
                    });
                    known_digits.insert(digit, set);
                }
            }
        });

    line.split(' ')
        .filter(|word| !word.is_empty() && *word != "|")
        .map(|word| match identify_digit(word) {
            Some(digit) => digit,
            None => match word.len() {
                // can be 2,3,5
                5 => {
                    todo!()
                }
                // can be 0,6,9
                6 => {
                    todo!()
                }
                _ => {
                    todo!()
                }
            },
        });

    todo!()
}

#[cfg(test)]
mod test_day {
    use super::{decode_line, part_1};

    const TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        let result = part_1(TEST_INPUT);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_2() {
        let input =
            r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let wires_map = decode_line(input);
    }
}
