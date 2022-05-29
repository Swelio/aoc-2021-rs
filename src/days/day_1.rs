//! This is the day 1 module.
//! The instructions are available here: https://adventofcode.com/2021/day/1

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 1;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let input_lines: Vec<usize> = BufReader::new(fs::File::open(input_path)?)
            .lines()
            .map(|l| l.unwrap().parse::<usize>().unwrap())
            .collect();

        let part_1_result = part_1(&input_lines);
        println!("Depth measurement increase count: {}", part_1_result);

        let part_2_result = part_2(&input_lines);
        println!("Windowed measurements increase count: {}", part_2_result);
        Ok(())
    }
}

fn count_increases(items: &[usize]) -> usize {
    items.windows(2).filter(|x| x[0] < x[1]).count()
}

fn part_1(lines: &[usize]) -> usize {
    count_increases(lines)
}

fn part_2(lines: &[usize]) -> usize {
    let windows: Vec<usize> = lines.windows(3).map(|x| x.iter().sum()).collect();
    count_increases(&windows)
}

#[cfg(test)]
mod test_day {
    use super::{part_1, part_2};

    /// Sample lines from guidelines
    const TEST_LINES: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {
        let input_lines: Vec<usize> = TEST_LINES
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        assert_eq!(part_1(&input_lines), 7);
    }

    #[test]
    fn test_part_2() {
        let input_lines: Vec<usize> = TEST_LINES
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        assert_eq!(part_2(&input_lines), 5);
    }
}
