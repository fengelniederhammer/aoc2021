use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    fold_once(read_file("src/day13/input.txt"))
}

pub fn get_answer_2() -> String {
    fold_all(read_file("src/day13/input.txt"))
}

fn read_file(path: &str) -> (HashSet<Point>, Vec<Fold>) {
    let file_content = read_to_string(path).expect("hoppla");

    file_content
        .lines()
        .fold((HashSet::new(), vec![]), |(mut points, mut folds), line| {
            match line.starts_with("fold along") {
                true => folds.push(Fold::from(line)),
                false if line.len() > 0 => {
                    points.insert(Point::from(line));
                }
                _ => {}
            }
            (points, folds)
        })
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point(usize, usize);

impl From<&str> for Point {
    fn from(str: &str) -> Self {
        let parts = str.split(",").collect::<Vec<&str>>();
        Point(
            parts[0].parse::<usize>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<&str> for Fold {
    fn from(str: &str) -> Self {
        let parts = str.split("=").collect::<Vec<&str>>();
        let n = parts[1].parse::<usize>().unwrap();
        match parts[0].contains("x") {
            true => Fold::X(n),
            false => Fold::Y(n),
        }
    }
}

impl Fold {
    fn transform(&self, point: Point) -> Point {
        match self {
            Fold::X(n) => match point.0.cmp(n) {
                Ordering::Less => point,
                _ => Point(2 * n - point.0, point.1),
            },
            Fold::Y(n) => match point.1.cmp(n) {
                Ordering::Less => point,
                _ => Point(point.0, 2 * n - point.1),
            },
        }
    }
}

fn fold_once((points, folds): (HashSet<Point>, Vec<Fold>)) -> usize {
    points
        .into_iter()
        .map(|point| folds[0].transform(point))
        .collect::<HashSet<Point>>()
        .len()
}

fn fold_all((mut points, folds): (HashSet<Point>, Vec<Fold>)) -> String {
    for fold in folds {
        points = points
            .into_iter()
            .map(|point| fold.transform(point))
            .collect::<HashSet<Point>>();
    }

    let size_x = points.iter().map(|point| point.0).max().unwrap() + 1;
    let size_y = points.iter().map(|point| point.1).max().unwrap() + 1;

    let mut result = vec![vec![" "; size_x]; size_y];
    for point in points {
        result[point.1][point.0] = "*";
    }

    result
        .into_iter()
        .map(|line| line.concat())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::day13::*;

    #[test]
    fn test_read_file() {
        let (points, folds) = read_file("src/day13/input.txt");

        assert_eq!(Some(&Point(802, 891)), points.get(&Point(802, 891)));
        assert_eq!(Some(&Fold::X(655)), folds.first());
    }

    #[test]
    fn test_count() {
        let input = read_file("src/day13/test_input.txt");

        let result = fold_once(input);

        assert_eq!(17, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(693, get_answer_1());
        println!("{}", get_answer_2());
    }
}
