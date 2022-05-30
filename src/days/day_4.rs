//! This is the day 4 module.
//! The instructions are available here: https://adventofcode.com/2021/day/4

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 4;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let input_lines: Vec<String> = normalize_lines(
            BufReader::new(fs::File::open(input_path)?)
                .lines()
                .map(|l| l.unwrap()),
        );
        let pick_list = get_pick_list(&input_lines);
        let grids = generate_grids(&input_lines[1..]);

        let part_1_result = part_1(&pick_list, grids.clone());
        println!("First winner grid score: {}", part_1_result);

        let part_2_result = part_2(&pick_list, grids);
        println!("Last winner grid score: {}", part_2_result);

        Ok(())
    }
}

#[derive(Clone)]
struct BingoGrid {
    // grid with each value as (number, is_marked)
    grid: [[(usize, bool); 5]; 5],
}

impl BingoGrid {
    fn from_lines(lines: &[String]) -> Self {
        if lines.len() != 5 {
            panic!("invalid chunk size: {:?}", lines);
        }

        let mut new_grid = [[(0usize, false); 5]; 5];

        for (row_index, line) in lines.iter().enumerate() {
            for (column_index, number) in line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
            {
                new_grid[row_index][column_index] = (number, false);
            }
        }

        Self { grid: new_grid }
    }

    fn mark_number(&mut self, number: usize) {
        self.grid.iter_mut().flatten().for_each(|x| {
            if x.0 == number {
                x.1 = true;
            }
        });
    }

    fn is_winner(&self) -> bool {
        for column_index in 0..5 {
            if self.grid.iter().all(|row| row[column_index].1) {
                return true;
            }
        }

        for row in self.grid.iter() {
            if row.iter().all(|x| x.1) {
                return true;
            }
        }

        false
    }

    fn get_unmarked_sum(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|(_, marked)| !(*marked))
            .map(|(number, _)| number)
            .sum()
    }
}

fn normalize_lines<I>(raw_lines: I) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    raw_lines
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn get_pick_list(lines: &[String]) -> Vec<usize> {
    lines[0]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn generate_grids(lines: &[String]) -> Vec<BingoGrid> {
    let mut grids = Vec::new();
    for chunk in lines.chunks(5) {
        let new_grid = BingoGrid::from_lines(chunk);
        grids.push(new_grid);
    }

    grids
}

fn part_1(pick_list: &[usize], mut grids: Vec<BingoGrid>) -> usize {
    for &number in pick_list {
        for grid in grids.iter_mut() {
            grid.mark_number(number);
            if grid.is_winner() {
                return number * grid.get_unmarked_sum();
            }
        }
    }

    panic!("no winner grid");
}

fn part_2(pick_list: &[usize], mut grids: Vec<BingoGrid>) -> usize {
    let mut winner_scores = Vec::new();
    let mut winner_indexes = Vec::new();

    for &number in pick_list {
        for (index, grid) in grids.iter_mut().enumerate() {
            if winner_indexes.contains(&index) {
                continue;
            }

            grid.mark_number(number);

            if grid.is_winner() {
                winner_scores.push(number * grid.get_unmarked_sum());
                winner_indexes.push(index);
            }
        }
    }

    winner_scores.pop().expect("no winner found")
}

#[cfg(test)]
mod test_day {
    use super::{generate_grids, get_pick_list, normalize_lines, part_1, part_2, BingoGrid};

    /// Sample lines from guidelines
    const TEST_LINES: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_pick_list() {
        let input_lines: Vec<String> = normalize_lines(TEST_LINES.lines().map(String::from));
        assert_eq!(
            get_pick_list(&input_lines),
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
    }

    #[test]
    fn test_grid_sum() {
        let marked_numbers: Vec<usize> = vec![14, 21, 17, 4, 9, 23, 11, 5, 2, 0, 7, 24];
        let input_grid: Vec<String> = normalize_lines(
            "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
                .lines()
                .map(String::from),
        );
        let mut grid = BingoGrid::from_lines(&input_grid);

        marked_numbers.iter().for_each(|&x| grid.mark_number(x));
        assert!(grid.is_winner());
        assert_eq!(grid.get_unmarked_sum(), 188);
    }

    #[test]
    fn test_part_1() {
        let input_lines: Vec<String> = normalize_lines(TEST_LINES.lines().map(String::from));
        let pick_list = get_pick_list(&input_lines);
        let grids = generate_grids(&input_lines[1..]);

        assert_eq!(part_1(&pick_list, grids), 4512);
    }

    #[test]
    fn test_part_2() {
        let input_lines: Vec<String> = normalize_lines(TEST_LINES.lines().map(String::from));
        let pick_list = get_pick_list(&input_lines);
        let grids = generate_grids(&input_lines[1..]);

        assert_eq!(part_2(&pick_list, grids), 1924);
    }
}
