//! This is the day 3 module.
//! The instructions are available here: https://adventofcode.com/2021/day/3

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 3;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let input_lines: Vec<String> = BufReader::new(fs::File::open(input_path)?)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let position_counters = count_bit_positions(&input_lines);

        let part_1_result = part_1(&position_counters);
        println!("Power consumption is: {}", part_1_result);

        let part_2_result = part_2(&input_lines);
        println!("Life support rating is: {}", part_2_result);

        Ok(())
    }
}

/// Structure the report as a binary tree with each node as CharNode.
struct CharNode {
    label: char,
    count: usize,
    zero_side: Option<Box<CharNode>>,
    one_side: Option<Box<CharNode>>,
}

impl CharNode {
    // Char at root
    const ROOT_CHAR: char = '#';

    fn root() -> Self {
        Self {
            label: Self::ROOT_CHAR,
            count: 0,
            zero_side: None,
            one_side: None,
        }
    }

    fn add_word<I>(&mut self, mut new_word: I)
    where
        I: Iterator<Item = char>,
    {
        let new_char = match new_word.next() {
            None => return,
            Some(new_char) => new_char,
        };

        let mut next_node = match new_char {
            '0' => &mut self.zero_side,
            '1' => &mut self.one_side,
            _ => panic!("unknown binary char '{}'", new_char),
        }
        .get_or_insert_with(|| {
            Box::new(Self {
                label: new_char,
                count: 0,
                zero_side: None,
                one_side: None,
            })
        });
        next_node.count += 1;

        next_node.add_word(new_word)
    }
}

fn count_bit_positions(lines: &[String]) -> Vec<(usize, usize)> {
    // An array with counters for each index as (0 count, 1 count)
    let mut position_counters = Vec::new();

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            if position_counters.len() <= index {
                position_counters.push((0, 0));
            }
            match char {
                '0' => position_counters[index].0 += 1,
                '1' => position_counters[index].1 += 1,
                _ => panic!("unknown char '{}'", char),
            }
        }
    }

    position_counters
}

/// Build an unsigned from provided tree, choosing the direction according to f function.
/// Return a tuple as (unsigned_result, depth)
fn evaluate_rating<F>(current_node: &CharNode, mut use_side_one: F) -> (usize, usize)
where
    F: FnMut(usize, usize) -> bool,
{
    let current_bit = match current_node.label {
        '0' => 0b0,
        '1' => 0b1,
        '#' => 0b0,
        _ => panic!("unknown bit char '{}'", current_node.label),
    };
    let (sub_rating, depth) = match (&current_node.zero_side, &current_node.one_side) {
        (None, None) => (0b0, 0),
        (Some(zero_node), Some(one_node)) => {
            if use_side_one(zero_node.count, one_node.count) {
                evaluate_rating(one_node, use_side_one)
            } else {
                evaluate_rating(zero_node, use_side_one)
            }
        }
        (Some(zero_node), None) => evaluate_rating(zero_node, use_side_one),
        (None, Some(one_node)) => evaluate_rating(one_node, use_side_one),
    };

    ((current_bit << depth) | sub_rating, depth + 1)
}

fn part_1(position_counters: &[(usize, usize)]) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;

    for (zero_counter, one_counter) in position_counters {
        // left shift both gamma and epsilon to meet bits size
        gamma <<= 1;
        epsilon <<= 1;

        // If the most common bit is 1, then we can set it on gamma
        // On the other hand, we set a bit 1 on epsilon if 1 is the least common bit
        if zero_counter < one_counter {
            gamma |= 0b1;
        } else {
            epsilon |= 0b1;
        }
    }

    usize::try_from(gamma * epsilon).unwrap()
}

fn part_2(lines: &[String]) -> usize {
    let mut tree_root = CharNode::root();

    for line in lines {
        tree_root.add_word(line.chars());
    }

    let (oxygen_generator_rating, _) =
        evaluate_rating(&tree_root, |zero_side, one_side| one_side >= zero_side);
    let (co2_scrubber_rating, _) =
        evaluate_rating(&tree_root, |zero_side, one_side| one_side < zero_side);

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod test_day {
    use super::{count_bit_positions, part_1, part_2};

    /// Sample lines from guidelines
    const TEST_LINES: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part_1() {
        let input_lines: Vec<String> = TEST_LINES.lines().map(String::from).collect();
        let position_counter = count_bit_positions(&input_lines);

        assert_eq!(part_1(&position_counter), 198);
    }

    #[test]
    fn test_part_2() {
        let input_lines: Vec<String> = TEST_LINES.lines().map(String::from).collect();

        assert_eq!(part_2(&input_lines), 230);
    }
}
