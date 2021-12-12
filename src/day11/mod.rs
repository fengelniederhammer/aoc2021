use std::collections::HashMap;
use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    count_flashes(read_file("src/day11/input.txt"))
}

pub fn get_answer_2() -> usize {
    find_synchronization_step(read_file("src/day11/input.txt"))
}

fn read_file(path: &str) -> HashMap<Point, usize> {
    let file_content = read_to_string(path).expect("hoppla");

    let mut map = HashMap::new();
    for (j, line) in file_content.lines().enumerate() {
        for (i, char) in line.chars().enumerate() {
            map.insert(
                (i as isize, j as isize),
                char.to_string().parse::<usize>().unwrap(),
            );
        }
    }
    map
}

type Point = (isize, isize);

fn count_flashes(mut input: HashMap<Point, usize>) -> usize {
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += do_step_and_get_flashing_points(&mut input).len();
    }

    flashes
}

fn find_synchronization_step(mut input: HashMap<Point, usize>) -> usize {
    let mut step = 1;

    while do_step_and_get_flashing_points(&mut input).len() != 100 {
        step += 1;
    }

    step
}

fn do_step_and_get_flashing_points(mut input: &mut HashMap<Point, usize>) -> Vec<Point> {
    input.iter_mut().for_each(|(_, energy)| *energy += 1);

    let mut flashing_points = find_new_flashing_points(&mut input, &vec![]);

    let mut new_flashing_points = flashing_points.clone();
    while !new_flashing_points.is_empty() {
        for point in &new_flashing_points {
            for neighbour in neighbouring_points_of(*point) {
                input.entry(neighbour).and_modify(|energy| *energy += 1);
            }
        }

        new_flashing_points = find_new_flashing_points(&input, &flashing_points);
        flashing_points.append(&mut new_flashing_points.clone())
    }

    input
        .iter_mut()
        .filter(|(point, _)| flashing_points.contains(point))
        .for_each(|(_, energy)| *energy = 0);
    flashing_points
}

fn find_new_flashing_points(
    input: &HashMap<Point, usize>,
    flashing_points: &[Point],
) -> Vec<Point> {
    input
        .iter()
        .filter(|(point, energy)| energy > &&9 && !flashing_points.contains(point))
        .map(|(point, _)| *point)
        .collect::<Vec<Point>>()
}

fn neighbouring_points_of((i, j): Point) -> Vec<Point> {
    vec![
        (i + 1, j),
        (i + 1, j + 1),
        (i, j + 1),
        (i - 1, j + 1),
        (i - 1, j),
        (i - 1, j - 1),
        (i, j - 1),
        (i + 1, j - 1),
    ]
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn test_read_file() {
        let matrix = read_file("src/day11/input.txt");

        assert_eq!(5, matrix[&(0, 0)]);
        assert_eq!(3, matrix[&(9, 9)]);
    }

    #[test]
    fn test_count() {
        let input = read_file("src/day11/test_input.txt");

        let result = count_flashes(input);

        assert_eq!(1656, result);
    }

    #[test]
    fn test_find_synchronization_step() {
        let input = read_file("src/day11/test_input.txt");

        let result = find_synchronization_step(input);

        assert_eq!(195, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(1617, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(258, get_answer_2());
    }
}
