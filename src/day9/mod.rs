use std::collections::HashMap;
use std::fs::read_to_string;

use crate::day9::Direction::{Down, Left, Right, Up};
use crate::day9::InnerState::{Inner, NoneYet};
use nalgebra::{DMatrix, RowDVector};

pub fn get_answer_1() -> usize {
    sum_low_point_risk(read_file("src/day9/input.txt"))
}

pub fn get_answer_2() -> usize {
    find_3_largest_basins(read_file("src/day9/input.txt"))
}

fn read_file(path: &str) -> DMatrix<usize> {
    let file_content = read_to_string(path).expect("hoppla");

    DMatrix::from_rows(
        &file_content
            .lines()
            .map(parse_line)
            .collect::<Vec<RowDVector<usize>>>(),
    )
}

fn parse_line(line: &str) -> RowDVector<usize> {
    RowDVector::from_iterator(
        line.len(),
        line.chars()
            .map(|c| c.to_string().parse::<usize>().unwrap()),
    )
}

fn sum_low_point_risk(input: DMatrix<usize>) -> usize {
    let matrix = to_hash_map(input);

    matrix
        .iter()
        .filter(|((i, j), height)| is_low_point(&matrix, *i, *j, height))
        .map(|(_, height)| height + 1)
        .sum()
}

fn to_hash_map(input: DMatrix<usize>) -> HashMap<Point, usize> {
    let (rows, columns) = input.shape();

    let mut matrix = HashMap::new();
    for j in 0..rows {
        for i in 0..columns {
            matrix.insert((i as isize, j as isize), input[(j, i)]);
        }
    }
    matrix
}

fn is_low_point(input: &HashMap<Point, usize>, row: isize, column: isize, height: &usize) -> bool {
    let left = input.get(&(row - 1, column)).unwrap_or(&10);
    let right = input.get(&(row + 1, column)).unwrap_or(&10);
    let top = input.get(&(row, column + 1)).unwrap_or(&10);
    let bottom = input.get(&(row, column - 1)).unwrap_or(&10);

    height < &left && height < right && height < top && height < &bottom
}

fn find_3_largest_basins(input: DMatrix<usize>) -> usize {
    let matrix = to_hash_map(input);

    let low_points: HashMap<Point, usize> = matrix
        .iter()
        .filter(|((i, j), height)| is_low_point(&matrix, *i, *j, height))
        .map(|(p, height)| (*p, *height))
        .collect();

    let mut covered_points = vec![];
    let mut basins = vec![];
    for (low_point, _) in low_points {
        if covered_points.contains(&low_point) {
            continue;
        }
        let basin_points =
            BasinIterator::from_point(low_point, matrix.clone()).collect::<Vec<Point>>();

        covered_points.append(&mut basin_points.clone());
        basins.push(basin_points.len());
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

type Point = (isize, isize);

#[derive(Debug)]
struct BasinIterator {
    matrix: HashMap<Point, usize>,
    origin: Point,
    directions_to_cover: Vec<Direction>,
    inner: InnerState,
    already_covered: Vec<Point>,
}

impl BasinIterator {
    fn from_point(origin: Point, matrix: HashMap<Point, usize>) -> BasinIterator {
        BasinIterator {
            origin,
            matrix,
            directions_to_cover: vec![Up, Left, Down, Right],
            inner: InnerState::NoneYet,
            already_covered: vec![],
        }
    }
}

impl Iterator for BasinIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            NoneYet => {
                let (i, j) = self.origin;

                let next_direction = match self.directions_to_cover.pop() {
                    None => {
                        self.already_covered.push(self.origin);

                        self.inner = InnerState::NoneLeft;
                        return Some(self.origin);
                    }
                    Some(next_direction) => next_direction,
                };
                let next = match next_direction {
                    Up => (i, j + 1),
                    Left => (i - 1, j),
                    Down => (i, j - 1),
                    Right => (i + 1, j),
                };

                if matches!(self.matrix.get(&next), None | Some(9))
                    || self.already_covered.contains(&next)
                {
                    return self.next();
                }

                self.already_covered.push(self.origin);

                self.inner = Inner(Box::from(BasinIterator {
                    matrix: self.matrix.clone(),
                    origin: next,
                    directions_to_cover: vec![Up, Left, Down, Right],
                    inner: NoneYet,
                    already_covered: self.already_covered.clone(),
                }));
                Some(self.origin)
            }
            Inner(inner) => match inner.next() {
                None => {
                    for point in &inner.already_covered {
                        if !self.already_covered.contains(&point) {
                            self.already_covered.push(point.clone());
                        }
                    }

                    match self.directions_to_cover.pop() {
                        None => None,
                        Some(next_direction) => {
                            let (i, j) = self.origin;

                            let next = match next_direction {
                                Up => (i, j + 1),
                                Left => (i - 1, j),
                                Down => (i, j - 1),
                                Right => (i + 1, j),
                            };

                            if matches!(self.matrix.get(&next), None | Some(9))
                                || self.already_covered.contains(&next)
                            {
                                return self.next();
                            }

                            self.inner = Inner(Box::from(BasinIterator {
                                matrix: self.matrix.clone(),
                                origin: next,
                                directions_to_cover: vec![Up, Left, Down, Right],
                                inner: NoneYet,
                                already_covered: self.already_covered.clone(),
                            }));
                            self.next()
                        }
                    }
                }
                some => {
                    for point in &inner.already_covered {
                        if !self.already_covered.contains(&point) {
                            self.already_covered.push(point.clone());
                        }
                    }

                    some
                }
            },
            InnerState::NoneLeft => None,
        }
    }
}

#[derive(Debug)]
enum InnerState {
    NoneYet,
    NoneLeft,
    Inner(Box<BasinIterator>),
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn test_read_file() {
        let matrix = read_file("src/day9/input.txt");

        assert_eq!(6, matrix[(0, 0)]);
        assert_eq!(7, matrix[(99, 99)]);
    }

    #[test]
    fn test_count() {
        let input = read_file("src/day9/test_input.txt");

        let result = sum_low_point_risk(input);

        assert_eq!(15, result);
    }

    #[test]
    fn test_sum_everything() {
        let input = read_file("src/day9/test_input.txt");

        let result = find_3_largest_basins(input);

        assert_eq!(1134, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(508, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(1564640, get_answer_2());
    }
}
