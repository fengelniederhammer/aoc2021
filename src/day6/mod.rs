use std::collections::HashMap;
use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    grow(read_file("src/day6/input.txt"), 80)
}

pub fn get_answer_2() -> usize {
    grow(read_file("src/day6/input.txt"), 256)
}

fn read_file(path: &str) -> Vec<u8> {
    let file_content = read_to_string(path).expect("hoppla");

    file_content
        .split(",")
        .map(|str| str.parse::<u8>().unwrap())
        .collect()
}

fn grow(input: Vec<u8>, days: i32) -> usize {
    let mut fishies_by_days = group_values(input);

    for _ in 0..days {
        let zeros = fishies_by_days.get(&0).cloned().unwrap_or(0);
        let sevens = fishies_by_days.get(&7).cloned().unwrap_or(0);

        fishies_by_days = fishies_by_days
            .iter()
            .map(|(n, number_of_fish)| match n {
                &0 => (6, *number_of_fish + sevens),
                &7 => (6, *number_of_fish + zeros),
                n => (*n - 1, *number_of_fish),
            })
            .collect();

        fishies_by_days.insert(8, zeros);
    }

    fishies_by_days.values().sum()
}

fn group_values(input: Vec<u8>) -> HashMap<u8, usize> {
    input.iter().fold(HashMap::new(), |mut map, n| {
        map.entry(*n).and_modify(|v| *v += 1).or_insert(1);
        map
    })
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn test_read_file() {
        let lines = read_file("src/day6/input.txt");

        assert_eq!(Some(&3), lines.first());
        assert_eq!(Some(&4), lines.last());
    }

    #[test]
    fn test_grow() {
        let input = read_file("src/day6/test_input.txt");

        assert_eq!(5, grow(input.clone(), 1));
        assert_eq!(26, grow(input.clone(), 18));
        assert_eq!(5934, grow(input.clone(), 80));
        assert_eq!(26984457539, grow(input, 256));
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(351188, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(1595779846729, get_answer_2());
    }
}
