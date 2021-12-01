use std::cmp::Ordering;
use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    get_increase_count(read_file())
}

pub fn get_answer_2() -> usize {
    get_increase_windowed_count(read_file())
}

fn read_file() -> Vec<i32> {
    let file_content = read_to_string("src/day1/input.txt").expect("hoppla");

    file_content.lines()
        .into_iter()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

fn get_increase_count(input: Vec<i32>) -> usize {
    input.iter()
        .zip(input.iter().skip(1))
        .map(|(first, second)|first.cmp(&second))
        .filter(|cmp| cmp == &Ordering::Less)
        .count()
}

fn get_increase_windowed_count(input: Vec<i32>) -> usize {
    let windowed_sum = input.windows(3)
        .map(|window| window.iter().sum())
        .collect();

    get_increase_count(windowed_sum)
}

#[cfg(test)]
mod tests {
    use crate::day1::*;
    use rstest::*;

    #[test]
    fn test_read_file() {
        let results = read_file();

        assert_eq!(2000, results.len());
        assert_eq!(Some(&141), results.first());
        assert_eq!(Some(&2682), results.last());
    }

    #[rstest]
    #[case(vec![1], 0)]
    #[case(vec![1, 1], 0)]
    #[case(vec![2, 1], 0)]
    #[case(vec![1, 2], 1)]
    #[case(vec![3, 1, 2], 1)]
    #[case(vec![3, 2, 1], 0)]
    #[case(vec![1, 2, 3], 2)]
    #[case(vec![141, 152, 164, 163, 164, 179, 210], 5)]
    fn test_get_increase_count(#[case] input: Vec<i32>, #[case] expected: usize) {
        let result = get_increase_count(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(vec![1], 0)]
    #[case(vec![1, 2], 0)]
    #[case(vec![3, 1, 2], 0)]
    #[case(vec![1, 1, 1, 3], 1)]
    #[case(vec![1, 1, 3, 1], 0)]
    #[case(vec![1, 1, 3, 1, 3], 1)]
    #[case(vec![1, 1, 3, 2, 4], 2)]
    #[case(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 5)]
    fn test_get_increase_windowed_count(#[case] input: Vec<i32>, #[case] expected: usize) {
        let result = get_increase_windowed_count(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        println!("{}", get_answer_2());
    }
}