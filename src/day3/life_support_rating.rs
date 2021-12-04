use crate::day3::bits_to_decimal;

pub fn find_life_support_rating(input: Vec<Vec<isize>>) -> isize {
    let oxygen_generator_rating = reduce_to_most_common_value_at_position(input.clone(), 0);
    let co2_scrubber_rating = reduce_to_least_common_value_at_position(input, 0);

    bits_to_decimal(oxygen_generator_rating) * bits_to_decimal(co2_scrubber_rating)
}

fn reduce_to_most_common_value_at_position(mut input: Vec<Vec<isize>>, position: usize) -> Vec<isize> {
    if all_values_are_equal(&input) {
        return input.pop().unwrap();
    }

    let bit_sum_at_position = sum_bits_at_position(&mut input, position);

    let most_common_bit = match 2 * bit_sum_at_position >= input.len() as isize {
        true => 1,
        false => 0,
    };

    let next = filter_by_bit(&mut input, position, most_common_bit);

    reduce_to_most_common_value_at_position(next, position + 1)
}

fn reduce_to_least_common_value_at_position(mut input: Vec<Vec<isize>>, position: usize) -> Vec<isize> {
    if all_values_are_equal(&input) {
        return input.pop().unwrap();
    }

    let bit_sum_at_position = sum_bits_at_position(&mut input, position);

    let least_common_bit = match 2 * bit_sum_at_position < input.len() as isize {
        true => 1,
        false => 0,
    };

    let next = filter_by_bit(&mut input, position, least_common_bit);

    reduce_to_least_common_value_at_position(next, position + 1)
}

fn all_values_are_equal(input: &Vec<Vec<isize>>) -> bool {
    input.iter()
        .zip(input.iter().skip(1))
        .fold(
            true,
            |all_values_are_equal, (first, second)| all_values_are_equal && first == second,
        )
}

fn sum_bits_at_position(input: &mut Vec<Vec<isize>>, position: usize) -> isize {
    input.iter().fold(
        0,
        |sum, bits| sum + bits.get(position).unwrap(),
    )
}

fn filter_by_bit(input: &mut Vec<Vec<isize>>, position: usize, most_common_bit: isize) -> Vec<Vec<isize>> {
    input.iter()
        .filter(|bits| *bits.get(position).unwrap() == most_common_bit)
        .cloned()
        .collect()
}
