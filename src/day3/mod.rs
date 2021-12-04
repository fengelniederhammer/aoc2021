mod life_support_rating;

use std::fs::read_to_string;
use crate::day3::life_support_rating::*;

pub fn get_answer_1() -> isize {
    do_it(read_file())
}

pub fn get_answer_2() -> isize {
    find_life_support_rating(read_file())
}

fn read_file() -> Vec<Vec<isize>> {
    let file_content = read_to_string("src/day3/input.txt").expect("hoppla");

    file_content.lines()
        .into_iter()
        .map(|str|
            str.chars().map(|char|
                char.to_string().parse::<isize>().unwrap()
            ).collect()
        )
        .collect()
}

fn do_it(input: Vec<Vec<isize>>) -> isize {
    let half_input_size = input.len() / 2;

    let gamma_rate_bit_counts = input.iter()
        .fold(
            vec![0; input.first().unwrap().len()],
            |gamma_rate_bit_counts, line| increase_bit_counts(gamma_rate_bit_counts, line),
        );

    let gamma_rate_bits: Vec<isize> = gamma_rate_bit_counts.iter()
        .map(|bit_count| match bit_count > &(half_input_size as isize) {
            true => 1,
            false => 0,
        })
        .collect();

    let epsilon_rate_bits: Vec<isize> = gamma_rate_bits.clone()
        .iter()
        .map(|bit| (bit - 1).abs())
        .collect();

    bits_to_decimal(gamma_rate_bits) * bits_to_decimal(epsilon_rate_bits)
}

fn increase_bit_counts(mut gamma_rate_bit_counts: Vec<isize>, line: &Vec<isize>) -> Vec<isize> {
    for (bit_position, bit) in line.iter().enumerate() {
        *gamma_rate_bit_counts.get_mut(bit_position).unwrap() += bit;
    }
    gamma_rate_bit_counts
}

fn bits_to_decimal(bits: Vec<isize>) -> isize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(position, bit)|(2_isize.pow(position as u32), bit))
        .fold(0, |decimal, (power, bit)| decimal + power * bit)
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::day3::*;
    use crate::day3::life_support_rating::find_life_support_rating;

    #[test]
    fn test_read_file() {
        let results = read_file();

        let expected_first = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1];
        let expected_last = vec![0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1];

        assert_eq!(1000, results.len());
        assert_eq!(Some(&expected_first), results.first());
        assert_eq!(Some(&expected_last), results.last());
    }

    #[rstest]
    #[case(vec ! [vec![1, 0], vec![1, 0], vec![0, 1]], 2)]
    #[case(vec ! [
    vec![0,0,1,0,0],
    vec![1,1,1,1,0],
    vec![1,0,1,1,0],
    vec![1,0,1,1,1],
    vec![1,0,1,0,1],
    vec![0,1,1,1,1],
    vec![0,0,1,1,1],
    vec![1,1,1,0,0],
    vec![1,0,0,0,0],
    vec![1,1,0,0,1],
    vec![0,0,0,1,0],
    vec![0,1,0,1,0],
    ], 198)]
    fn test_do_it(#[case] input: Vec<Vec<isize>>, #[case] expected: isize) {
        let result = do_it(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(vec ! [vec![1, 0], vec![1, 0], vec![0, 1]], 2)]
    #[case(vec ! [vec![1, 1, 0], vec![1, 0, 0], vec![0, 1, 1]], 18)]
    #[case(vec ! [
    vec![0,0,1,0,0],
    vec![1,1,1,1,0],
    vec![1,0,1,1,0],
    vec![1,0,1,1,1],
    vec![1,0,1,0,1],
    vec![0,1,1,1,1],
    vec![0,0,1,1,1],
    vec![1,1,1,0,0],
    vec![1,0,0,0,0],
    vec![1,1,0,0,1],
    vec![0,0,0,1,0],
    vec![0,1,0,1,0],
    ], 230)]
    fn test_do_it_better(#[case] input: Vec<Vec<isize>>, #[case] expected: isize) {
        let result = find_life_support_rating(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(738234, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(3969126, get_answer_2());
    }
}