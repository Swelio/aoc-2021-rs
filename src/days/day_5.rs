//! This is the day 5 module.
//! The instructions are available here: https://adventofcode.com/2021/day/5

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use regex::RegexBuilder;

use crate::{DailySolution, Error};

pub struct Solution;

impl DailySolution for Solution {
    const DAY_NUMBER: u8 = 5;

    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let input_content = fs::read_to_string(input_path)?;
        let segments = Segment::from_lines(&input_content);

        let part_1_result = part_1(&segments);
        println!("Total overlap points of straight lines: {}", part_1_result);

        let part_2_result = part_2(&segments);
        println!("Total intersections: {}", part_2_result);

        Ok(())
    }
}

fn get_range(a: usize, b: usize) -> Vec<usize> {
    if a <= b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_range(self, other: Self) -> Vec<Self> {
        if self == other {
            return vec![self];
        }

        if self.x == other.x {
            get_range(self.y, other.y)
                .into_iter()
                .map(|y| Self { x: self.x, y })
                .collect()
        } else if self.y == other.y {
            get_range(self.x, other.x)
                .into_iter()
                .map(|x| Self { x, y: self.y })
                .collect()
        } else {
            let x_range = get_range(self.x, other.x);
            let y_range = get_range(self.y, other.y);

            x_range
                .into_iter()
                .zip(y_range.into_iter())
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment(Point, Point);

impl Segment {
    fn from_lines(input_content: &str) -> Vec<Segment> {
        let line_parser =
            RegexBuilder::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$")
                .multi_line(true)
                .build()
                .unwrap();

        line_parser
            .captures_iter(input_content)
            .map(|cap| {
                let x1 = cap["x1"].parse().unwrap();
                let y1 = cap["y1"].parse().unwrap();
                let x2 = cap["x2"].parse().unwrap();
                let y2 = cap["y2"].parse().unwrap();

                Segment(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
            })
            .collect()
    }

    fn get_range(&self) -> Vec<Point> {
        self.0.get_range(self.1)
    }
}

fn count_points(segments: &[Segment]) -> HashMap<Point, usize> {
    let mut counter = HashMap::new();

    for segment in segments {
        segment
            .get_range()
            .into_iter()
            .for_each(|point| *counter.entry(point).or_insert(0) += 1);
    }

    counter
}

fn count_intersections(counter: HashMap<Point, usize>) -> usize {
    counter.into_iter().filter(|(_, count)| *count >= 2).count()
}

fn part_1(segments: &[Segment]) -> usize {
    let counter = count_points(
        segments
            .iter()
            // keep only vertical segments
            .filter(|seg| seg.0.x == seg.1.x || seg.0.y == seg.1.y)
            .copied()
            .collect::<Vec<Segment>>()
            .as_slice(),
    );
    count_intersections(counter)
}

fn part_2(segments: &[Segment]) -> usize {
    let counter = count_points(segments);
    count_intersections(counter)
}

#[cfg(test)]
mod test_day {
    use std::collections::HashSet;

    use super::{count_intersections, count_points, part_1, part_2, Point, Segment};

    /// Sample lines from guidelines
    const TEST_LINES: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_diagonal_collinear() {
        let segments = Segment::from_lines("6,0 -> 0,6\n6,0 -> 3,3");
        assert_eq!(count_intersections(count_points(&segments)), 4);

        let segments = Segment::from_lines("0,0 -> 6,6\n0,0 -> 3,3");
        assert_eq!(count_intersections(count_points(&segments)), 4);
    }

    #[test]
    fn test_collinear() {
        let segments = Segment::from_lines("0,9 -> 5,9\n0,9 -> 2,9");
        assert_eq!(count_intersections(count_points(&segments)), 3);

        let segments = Segment::from_lines("0,9 -> 2,9\n2,9 -> 5,9");
        assert_eq!(count_intersections(count_points(&segments)), 1);

        let segments = Segment::from_lines("9,0 -> 9,5\n9,0 -> 9,2");
        assert_eq!(count_intersections(count_points(&segments)), 3);

        let segments = Segment::from_lines("9,4 -> 3,4\n3,4 -> 1,4\n2,2 -> 2,1\n7,0 -> 7,4");
        assert_eq!(count_intersections(count_points(&segments)), 2);
    }

    #[test]
    fn test_no_intersection() {
        let segments = Segment::from_lines("0,8 -> 3,8\n0,9 -> 3,9");
        assert_eq!(count_intersections(count_points(&segments)), 0);

        let segments = Segment::from_lines("0,5 -> 3,5\n4,5 -> 8,5");
        assert_eq!(count_intersections(count_points(&segments)), 0);

        let segments = Segment::from_lines("6,0 -> 4,2\n3,3 -> 2,4");
        assert_eq!(count_intersections(count_points(&segments)), 0);

        let segments = Segment::from_lines("0,0 -> 2,2\n3,3 -> 4,4");
        assert_eq!(count_intersections(count_points(&segments)), 0);
    }

    #[test]
    fn test_diagonal_cross() {
        let segments = Segment::from_lines("0,0 -> 6,6\n0,6 -> 6,0");
        assert_eq!(count_intersections(count_points(&segments)), 1);

        let segments = Segment::from_lines("0,0 -> 6,6\n3,6 -> 3,0");
        assert_eq!(count_intersections(count_points(&segments)), 1);
    }

    #[test]
    fn test_known_intersections_straight() {
        let segments: Vec<Segment> = Segment::from_lines(TEST_LINES)
            .into_iter()
            .filter(|seg| seg.0.x == seg.1.x || seg.0.y == seg.1.y)
            .collect();
        let known_intersections: HashSet<Point> = [(0, 9), (1, 9), (2, 9), (3, 4), (7, 4)]
            .into_iter()
            .map(|(x, y)| Point { x, y })
            .collect();
        let intersections: HashSet<Point> = count_points(&segments)
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(point, _)| point)
            .collect();

        assert_eq!(intersections.difference(&known_intersections).count(), 0);
    }

    #[test]
    fn test_known_intersections() {
        let segments: Vec<Segment> = Segment::from_lines(TEST_LINES);
        let known_intersections = [
            (0, 9),
            (1, 9),
            (2, 2),
            (2, 9),
            (3, 4),
            (4, 4),
            (5, 3),
            (5, 5),
            (6, 4),
            (7, 4),
            (7, 1),
            (7, 3),
        ]
        .into_iter()
        .map(|(x, y)| Point { x, y })
        .collect::<HashSet<Point>>();
        let intersections: HashSet<Point> = count_points(&segments)
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(point, _)| point)
            .collect();
        assert_eq!(intersections.difference(&known_intersections).count(), 0);
    }

    #[test]
    fn test_part_1() {
        let segments = Segment::from_lines(TEST_LINES);
        assert_eq!(part_1(&segments), 5);
    }

    #[test]
    fn test_part_2() {
        let segments = Segment::from_lines(TEST_LINES);
        assert_eq!(part_2(&segments), 12);
    }
}
