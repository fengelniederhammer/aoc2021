use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    compute_least_fuel_linear_consumption(read_file("src/day7/input.txt"))
}

pub fn get_answer_2() -> usize {
    compute_least_fuel_non_linear_consumption(read_file("src/day7/input.txt"))
}

fn read_file(path: &str) -> Vec<i32> {
    let file_content = read_to_string(path).expect("hoppla");

    file_content
        .split(",")
        .map(|str| str.parse::<i32>().unwrap())
        .collect()
}

fn compute_least_fuel_linear_consumption(input: Vec<i32>) -> usize {
    compute_least_fuel_consumption(input, Box::from(compute_fuel_to_position_linear))
}

fn compute_least_fuel_non_linear_consumption(input: Vec<i32>) -> usize {
    compute_least_fuel_consumption(input, Box::from(compute_fuel_to_position_non_linear))
}

fn compute_least_fuel_consumption(
    input: Vec<i32>,
    compute_fn: Box<dyn Fn(&Vec<i32>, i32) -> i32>,
) -> usize {
    (0..=*input.iter().max().unwrap())
        .into_iter()
        .map(|n| compute_fn(&input, n))
        .min()
        .unwrap() as usize
}

fn compute_fuel_to_position_linear(input: &Vec<i32>, position: i32) -> i32 {
    input.iter().map(|x| (x - position).abs()).sum()
}

fn compute_fuel_to_position_non_linear(input: &Vec<i32>, position: i32) -> i32 {
    input
        .iter()
        .map(|x| sum_of_1_to((x - position).abs()))
        .sum()
}

fn sum_of_1_to(n: i32) -> i32 {
    (1..=n).sum()
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    fn test_read_file() {
        let lines = read_file("src/day7/input.txt");

        assert_eq!(Some(&1101), lines.first());
        assert_eq!(Some(&82), lines.last());
    }

    #[test]
    fn test_linear() {
        let input = read_file("src/day7/test_input.txt");

        let result = compute_least_fuel_linear_consumption(input);

        assert_eq!(37, result);
    }

    #[test]
    fn test_non_linear() {
        let input = read_file("src/day7/test_input.txt");

        let result = compute_least_fuel_non_linear_consumption(input);

        assert_eq!(168, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(339321, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(95476244, get_answer_2());
    }
}
