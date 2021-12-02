use std::fs::read_to_string;

use crate::day2::Direction::Forward;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(())
        }
    }
}

impl Direction {
    fn apply(&self, value: i32, (x, depth): (isize, isize)) -> (isize, isize) {
        match self {
            Forward => (x + value as isize, depth),
            Direction::Down => (x, depth + value as isize),
            Direction::Up => (x, depth - value as isize),
        }
    }

    fn apply_better(&self, value: i32, (x, depth, aim): (isize, isize, isize)) -> (isize, isize, isize) {
        let value = value as isize;
        match self {
            Forward => (x + value as isize, depth + aim * value, aim),
            Direction::Down => (x, depth, aim + value),
            Direction::Up => (x, depth, aim - value),
        }
    }
}

pub fn get_answer_1() -> isize {
    do_it(read_file())
}

pub fn get_answer_2() -> isize {
    do_it_better(read_file())
}

fn read_file() -> Vec<(Direction, i32)> {
    let file_content = read_to_string("src/day2/input.txt").expect("hoppla");

    file_content.lines()
        .into_iter()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (Direction, i32) {
    let mut parts: Vec<&str> = line.split(" ").collect();

    let value = parts.pop().unwrap().parse::<i32>().unwrap();
    let direction = parts.pop().unwrap().try_into().unwrap();

    (direction, value)
}

fn do_it(input: Vec<(Direction, i32)>) -> isize {
    let final_position = input.iter().fold(
        (0, 0),
        |position, (direction, value)| direction.apply(*value, position),
    );

    final_position.0 * final_position.1
}

fn do_it_better(input: Vec<(Direction, i32)>) -> isize {
    let final_position = input.iter().fold(
        (0, 0, 0),
        |position_with_aim, (direction, value)| direction.apply_better(*value, position_with_aim),
    );

    final_position.0 * final_position.1
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::day2::*;

    #[test]
    fn test_read_file() {
        let results = read_file();

        assert_eq!(1000, results.len());
        assert_eq!(Some(&(Direction::Forward, 2)), results.first());
        assert_eq!(Some(&(Direction::Down, 9)), results.get(1));
        assert_eq!(Some(&(Direction::Up, 6)), results.get(2));
        assert_eq!(Some(&(Direction::Forward, 4)), results.last());
    }

    #[rstest]
    #[case(vec ! [], 0)]
    #[case(vec ! [(Direction::Forward, 1), (Direction::Down, 1)], 1)]
    #[case(vec ! [(Direction::Forward, 1), (Direction::Up, 1)], - 1)]
    #[case(vec ! [
    (Direction::Forward, 5),
    (Direction::Down, 5),
    (Direction::Forward, 8),
    (Direction::Up, 3),
    (Direction::Down, 8),
    (Direction::Forward, 2),
    ], 150)]
    fn test_do_it(#[case] input: Vec<(Direction, i32)>, #[case] expected: isize) {
        let result = do_it(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(vec ! [], 0)]
    #[case(vec ! [(Direction::Forward, 1), (Direction::Down, 1)], 0)]
    #[case(vec ! [(Direction::Down, 2), (Direction::Forward, 3)], 18)]
    #[case(vec ! [(Direction::Forward, 1), (Direction::Up, 1)], 0)]
    #[case(vec ! [(Direction::Up, 2), (Direction::Forward, 3)], - 18)]
    #[case(vec ! [
    (Direction::Forward, 5),
    (Direction::Down, 5),
    (Direction::Forward, 8),
    (Direction::Up, 3),
    (Direction::Down, 8),
    (Direction::Forward, 2),
    ], 900)]
    fn test_do_it_better(#[case] input: Vec<(Direction, i32)>, #[case] expected: isize) {
        let result = do_it_better(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        println!("{}", get_answer_2());
    }
}