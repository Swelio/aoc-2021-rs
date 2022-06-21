//! This is the day 6 module.
//! The instructions are available here: https://adventofcode.com/2021/day/6

use std::fs;
use std::path::Path;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 6;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut fish_net = FishNet::from_line(fs::read_to_string(input_path)?.as_str());

        let part_1_result = part_1(&mut fish_net);
        println!("Total fishes after 80 days: {}", part_1_result);

        let part_2_result = part_2(&mut fish_net);
        println!("Total fishes after 256 days: {}", part_2_result);
        Ok(())
    }
}

fn part_1(fish_net: &mut FishNet) -> usize {
    (0..80).for_each(|_| fish_net.roll_fishes());

    fish_net.fishes.iter().sum()
}

fn part_2(fish_net: &mut FishNet) -> usize {
    (80..256).for_each(|_| fish_net.roll_fishes());

    fish_net.fishes.iter().sum()
}

/// A structure to keep tracking over fishes of each age
struct FishNet {
    fishes: [usize; 9],
}

impl FishNet {
    fn new() -> Self {
        Self {
            fishes: [0usize; 9],
        }
    }

    fn from_line(line: &str) -> Self {
        let mut new_fish_net = Self::new();

        for fish_age in line.trim().split(',').filter_map(|item| item.parse().ok()) {
            new_fish_net.insert_fish(fish_age);
        }

        new_fish_net
    }

    fn insert_fish(&mut self, fish_age: usize) {
        if fish_age >= 9 {
            panic!("unexpected fish age {}", fish_age);
        }

        self.fishes[fish_age] += 1;
    }

    fn roll_fishes(&mut self) {
        let new_fishes = self.fishes[0];

        self.fishes[0..7].rotate_left(1);
        self.fishes[6] += self.fishes[7];
        self.fishes[7..9].rotate_left(1);
        self.fishes[8] = new_fishes;
    }
}

#[cfg(test)]
mod test_day {
    use super::FishNet;

    /// Sample lines from guidelines
    const TEST_LINES: &str = "3,4,3,1,2";

    #[test]
    fn test_growth() {
        let mut fish_net = FishNet::from_line(TEST_LINES);

        for grow_line in [
            vec![2, 3, 2, 0, 1],
            vec![1, 2, 1, 6, 0, 8],
            vec![0, 1, 0, 5, 6, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 7, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 7],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5],
            vec![1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8],
            vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 0, 4, 5, 6, 6, 6, 7, 7, 8, 8],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6, 3, 4, 5, 5, 5, 6, 6, 7, 7, 8],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7],
            vec![
                1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
            ],
            vec![
                0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
            ],
            vec![
                6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
            ],
        ] {
            fish_net.roll_fishes();
            assert_eq!(fish_net.fishes.iter().sum::<usize>(), grow_line.len());
        }
    }

    #[test]
    fn test_part_1() {
        let mut fish_net = FishNet::from_line(TEST_LINES);

        (0..80).for_each(|_| fish_net.roll_fishes());

        assert_eq!(fish_net.fishes.iter().sum::<usize>(), 5934);
    }

    #[test]
    fn test_part_2() {
        let mut fish_net = FishNet::from_line(TEST_LINES);

        (0..256).for_each(|_| fish_net.roll_fishes());

        assert_eq!(fish_net.fishes.iter().sum::<usize>(), 26984457539);
    }
}
