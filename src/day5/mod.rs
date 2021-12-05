use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::AddAssign;

pub fn get_answer_1() -> usize {
    count_non_diagonal_points_higher_than_2(read_file("src/day5/input.txt"))
}

pub fn get_answer_2() -> usize {
    count_all_points_higher_than_2(read_file("src/day5/input.txt"))
}

fn read_file(path: &str) -> Vec<Line> {
    let file_content = read_to_string(path).expect("hoppla");

    file_content
        .lines()
        .into_iter()
        .map(Line::from_str)
        .collect()
}

fn count_non_diagonal_points_higher_than_2(input: Vec<Line>) -> usize {
    let map = input
        .into_iter()
        .filter(Line::is_not_diagonal)
        .flat_map(|line| line.into_iter())
        .fold(HashMap::<Point, usize>::new(), |mut map, point| {
            map.entry(point)
                .and_modify(|value| *value += 1)
                .or_insert(1);
            map
        });

    map.iter().filter(|(_, value)| value > &&1).count()
}

fn count_all_points_higher_than_2(input: Vec<Line>) -> usize {
    let map = input.into_iter().flat_map(|line| line.into_iter()).fold(
        HashMap::<Point, usize>::new(),
        |mut map, point| {
            map.entry(point)
                .and_modify(|value| *value += 1)
                .or_insert(1);
            map
        },
    );

    map.iter().filter(|(_, value)| value > &&1).count()
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_str(str: &str) -> Point {
        let parts: Vec<&str> = str.split(",").collect();
        Point {
            x: parts[0].parse::<isize>().unwrap(),
            y: parts[1].parse::<isize>().unwrap(),
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn from_str(str_line: &str) -> Line {
        let parts: Vec<&str> = str_line.split(" -> ").collect();
        let from = Point::from_str(parts[0]);
        let to = Point::from_str(parts[1]);

        Line { from, to }
    }

    fn is_not_diagonal(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        let delta_x = self.to.x - self.from.x;
        let delta_y = self.to.y - self.from.y;

        if delta_x != 0 && delta_y != 0 && delta_x.abs() != delta_y.abs() {
            panic!("das sieht komisch aus: {:?}", self);
        }

        LineIterator {
            line: self,
            direction: Point {
                x: delta_x.signum(),
                y: delta_y.signum(),
            },
            finished: false,
            counter: 0,
        }
    }
}

#[derive(Debug)]
struct LineIterator {
    line: Line,
    direction: Point,
    finished: bool,
    counter: usize,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.counter > 1000 {
            None
        } else {
            let next = self.line.from.clone();
            self.counter += 1;
            if next == self.line.to {
                self.finished = true;
            } else {
                self.line.from += self.direction;
            }
            Some(next)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn test_read_file() {
        let lines = read_file("src/day5/input.txt");

        let expected_first = Line {
            from: Point { x: 284, y: 294 },
            to: Point { x: 733, y: 743 },
        };

        let expected_last = Line {
            from: Point { x: 595, y: 393 },
            to: Point { x: 941, y: 393 },
        };

        assert_eq!(500, lines.len());
        assert_eq!(Some(&expected_first), lines.first());
        assert_eq!(Some(&expected_last), lines.last());
    }

    #[test]
    fn test_input_non_diagonal_only() {
        let result = count_non_diagonal_points_higher_than_2(read_file("src/day5/test_input.txt"));

        assert_eq!(5, result);
    }

    #[test]
    fn test_input_all() {
        let result = count_all_points_higher_than_2(read_file("src/day5/test_input.txt"));

        assert_eq!(12, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(6572, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(21466, get_answer_2());
    }
}
