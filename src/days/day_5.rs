//! This is the day 5 module.
//! The instructions are available here: https://adventofcode.com/2021/day/5

use std::cmp::{max, min};
use std::collections::HashSet;
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

type CoordinateUnit = isize;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: CoordinateUnit,
    y: CoordinateUnit,
}

#[derive(Copy, Clone, Debug)]
struct Segment(Point, Point);

impl Segment {
    fn from_points(a: Point, b: Point) -> Self {
        if a <= b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }

    fn from_lines(input_content: &str) -> Vec<Segment> {
        let line_parser =
            RegexBuilder::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$")
                .multi_line(true)
                .build()
                .unwrap();

        line_parser
            .captures_iter(input_content)
            .map(|cap| {
                let x1 = cap["x1"].parse::<CoordinateUnit>().unwrap();
                let y1 = cap["y1"].parse::<CoordinateUnit>().unwrap();
                let x2 = cap["x2"].parse::<CoordinateUnit>().unwrap();
                let y2 = cap["y2"].parse::<CoordinateUnit>().unwrap();

                Segment::from_points(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
            })
            .collect()
    }

    /// Using this mathematical solution https://stackoverflow.com/a/55598451, we find intersections
    /// between segments.
    fn get_intersection(&self, other: &Self) -> Option<Vec<Point>> {
        let dx1 = (self.1.x - self.0.x) as f32;
        let dx2 = (other.1.x - other.0.x) as f32;
        let dy1 = (self.1.y - self.0.y) as f32;
        let dy2 = (other.1.y - other.0.y) as f32;
        let dx3 = (self.0.x - other.0.x) as f32;
        let dy3 = (self.0.y - other.0.y) as f32;

        let det = (dx1 * dy2 - dx2 * dy1) as f32;
        let det1 = (dx1 * dy3 - dx3 * dy1) as f32;
        let det2 = (dx2 * dy3 - dx3 * dy2) as f32;

        // segments are parallel
        if det == 0.0 {
            let start = max(self.0, other.0);
            let end = min(self.1, other.1);

            // segments are not collinear
            if det1 != 0.0 || det2 != 0.0 || start > end {
                return None;
            }

            return Some(if start.x == end.x {
                (start.y..=end.y).map(|y| Point { x: start.x, y }).collect()
            } else if start.y == end.y {
                (start.x..=end.x).map(|x| Point { x, y: start.y }).collect()
            } else {
                let x_range: Vec<CoordinateUnit> = if start.x < end.x {
                    (start.x..=end.x).collect()
                } else {
                    (end.x..=start.x).rev().collect()
                };
                let y_range: Vec<CoordinateUnit> = if start.y < end.y {
                    (start.y..=end.y).collect()
                } else {
                    (end.y..=start.y).rev().collect()
                };

                x_range
                    .into_iter()
                    .zip(y_range.into_iter())
                    .map(|(x, y)| Point { x, y })
                    .collect()
            });
        }

        let s_coeff = det1 / det;
        let t_coeff = det2 / det;

        if (0.0..=1.0).contains(&s_coeff) && (0.0..=1.0).contains(&t_coeff) {
            Some(vec![Point {
                x: ((self.0.x as f32) + t_coeff * dx1) as CoordinateUnit,
                y: ((self.0.y as f32) + t_coeff * dy1) as CoordinateUnit,
            }])
        } else {
            None
        }
    }
}

fn get_segments_intersections(segments: &[Segment]) -> Vec<Point> {
    segments
        .iter()
        .take(segments.len() - 1)
        .enumerate()
        .flat_map(|(index, seg_a)| {
            segments
                .iter()
                .skip(index + 1)
                .filter_map(|seg_b| seg_a.get_intersection(seg_b))
                .flatten()
        })
        .collect::<HashSet<Point>>()
        .into_iter()
        .collect()
}

fn part_1(segments: &[Segment]) -> usize {
    get_segments_intersections(
        segments
            .iter()
            // keep only vertical segments
            .filter(|seg| seg.0.x == seg.1.x || seg.0.y == seg.1.y)
            .copied()
            .collect::<Vec<Segment>>()
            .as_slice(),
    )
    .len()
}

fn part_2(segments: &[Segment]) -> usize {
    get_segments_intersections(segments).len()
}

#[cfg(test)]
mod test_day {
    use std::collections::HashSet;

    use super::{get_segments_intersections, part_1, part_2, Point, Segment};

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
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 4);

        let segments = Segment::from_lines("0,0 -> 6,6\n0,0 -> 3,3");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 4);
    }

    #[test]
    fn test_collinear() {
        let segments = Segment::from_lines("0,9 -> 5,9\n0,9 -> 2,9");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 3);

        let segments = Segment::from_lines("0,9 -> 2,9\n2,9 -> 5,9");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 1);

        let segments = Segment::from_lines("9,0 -> 9,5\n9,0 -> 9,2");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 3);

        let segments = Segment::from_lines("9,4 -> 3,4\n3,4 -> 1,4\n2,2 -> 2,1\n7,0 -> 7,4");
        assert_eq!(
            segments
                .iter()
                .take(segments.len() - 1)
                .enumerate()
                .flat_map(|(index, seg_a)| {
                    segments
                        .iter()
                        .skip(index + 1)
                        .filter_map(|seg_b| seg_a.get_intersection(seg_b))
                        .flatten()
                        .collect::<HashSet<Point>>()
                })
                .count(),
            2
        );
    }

    #[test]
    fn test_no_intersection() {
        let segments = Segment::from_lines("0,8 -> 3,8\n0,9 -> 3,9");
        assert!(segments[0].get_intersection(&segments[1]).is_none());

        let segments = Segment::from_lines("0,5 -> 3,5\n4,5 -> 8,5");
        assert!(segments[0].get_intersection(&segments[1]).is_none());

        let segments = Segment::from_lines("6,0 -> 4,2\n3,3 -> 2,4");
        assert!(segments[0].get_intersection(&segments[1]).is_none());

        let segments = Segment::from_lines("0,0 -> 2,2\n3,3 -> 4,4");
        assert!(segments[0].get_intersection(&segments[1]).is_none());
    }

    #[test]
    fn test_diagonal_cross() {
        let segments = Segment::from_lines("0,0 -> 6,6\n0,6 -> 6,0");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 1);

        let segments = Segment::from_lines("0,0 -> 6,6\n3,6 -> 3,0");
        assert_eq!(segments[0].get_intersection(&segments[1]).unwrap().len(), 1);
    }

    #[test]
    fn test_known_intersections_straight() {
        let segments: Vec<Segment> = Segment::from_lines(TEST_LINES)
            .into_iter()
            .filter(|seg| seg.0.x == seg.1.x || seg.0.y == seg.1.y)
            .collect();
        let known_intersections = [(0, 9), (1, 9), (2, 9), (3, 4), (7, 4)]
            .into_iter()
            .map(|(x, y)| Point { x, y })
            .collect::<HashSet<Point>>();
        let intersections = get_segments_intersections(&segments)
            .into_iter()
            .collect::<HashSet<Point>>();
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
        let intersections = get_segments_intersections(&segments)
            .into_iter()
            .collect::<HashSet<Point>>();
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
