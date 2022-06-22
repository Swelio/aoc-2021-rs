//! This is the day 7 module.
//! The instructions are available here: https://adventofcode.com/2021/day/7

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 7;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let crab_positions = get_crab_positions(fs::read_to_string(input_path)?.as_str());

        let (best_position, fuel_cost) = part_1(&crab_positions);
        println!(
            "Best position is {} for {} fuel units",
            best_position, fuel_cost
        );

        let (best_position_2, fuel_cost_2) = part_2(&crab_positions);
        println!(
            "New best position is {} for {} fuel units",
            best_position_2, fuel_cost_2
        );

        Ok(())
    }
}

fn part_1(crab_positions: &HashMap<usize, usize>) -> (usize, usize) {
    (0..=*crab_positions.keys().max().unwrap())
        .map(|position| {
            let fuel_cost = crab_positions
                .iter()
                .map(|(other_position, other_count)| {
                    position.abs_diff(*other_position) * other_count
                })
                .sum::<usize>();
            (position, fuel_cost)
        })
        .min_by_key(|(_, fuel_cost)| *fuel_cost)
        .unwrap()
}

fn part_2(crab_positions: &HashMap<usize, usize>) -> (usize, usize) {
    (0..=*crab_positions.keys().max().unwrap())
        .map(|position| {
            let fuel_cost = crab_positions
                .iter()
                .map(|(other_position, other_count)| {
                    compute_position_cost(position, *other_position) * other_count
                })
                .sum::<usize>();
            (position, fuel_cost)
        })
        .min_by_key(|(_, fuel_cost)| *fuel_cost)
        .unwrap()
}

fn get_crab_positions(line: &str) -> HashMap<usize, usize> {
    let mut positions = HashMap::new();
    line.trim()
        .split(',')
        .for_each(|item| *positions.entry(item.parse::<usize>().unwrap()).or_insert(0) += 1);

    positions
}

fn compute_position_cost(origin: usize, destination: usize) -> usize {
    let diff = origin.abs_diff(destination);
    let mut acc = 0;

    (1..=diff).for_each(|cost| acc += cost);

    acc
}

#[cfg(test)]
mod test_day {
    use super::{compute_position_cost, get_crab_positions, part_1, part_2};

    const TEST_LINE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_position_cost_part_2() {
        assert_eq!(compute_position_cost(16, 5), 66);
        assert_eq!(compute_position_cost(1, 5), 10);
        assert_eq!(compute_position_cost(2, 5), 6);
        assert_eq!(compute_position_cost(0, 5), 15);
        assert_eq!(compute_position_cost(4, 5), 1);
        assert_eq!(compute_position_cost(2, 5), 6);
        assert_eq!(compute_position_cost(7, 5), 3);
        assert_eq!(compute_position_cost(1, 5), 10);
        assert_eq!(compute_position_cost(2, 5), 6);
        assert_eq!(compute_position_cost(14, 5), 45);
    }

    #[test]
    fn test_part_1() {
        let positions = get_crab_positions(TEST_LINE);
        let (best_position, fuel_cost) = part_1(&positions);
        assert_eq!(best_position, 2usize);
        assert_eq!(fuel_cost, 37usize);
    }

    #[test]
    fn test_part_2() {
        let positions = get_crab_positions(TEST_LINE);
        let (best_position, fuel_cost) = part_2(&positions);
        assert_eq!(best_position, 5usize);
        assert_eq!(fuel_cost, 168usize);
    }
}
