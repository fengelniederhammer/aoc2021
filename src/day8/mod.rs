use std::collections::HashMap;
use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    count_1_4_7_8(read_file("src/day8/input.txt"))
}

pub fn get_answer_2() -> usize {
    sum_display_values(read_file("src/day8/input.txt"))
}

fn read_file(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file_content = read_to_string(path).expect("hoppla");

    file_content.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    match &line.split(" | ").collect::<Vec<&str>>()[..] {
        [code, display] => (
            code.split_whitespace().map(String::from).collect(),
            display.split_whitespace().map(String::from).collect(),
        ),
        _ => panic!("unexpected input {:?}", line),
    }
}

fn count_1_4_7_8(input: Vec<(Vec<String>, Vec<String>)>) -> usize {
    input
        .iter()
        .flat_map(|(_, display)| display.iter())
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

fn sum_display_values(input: Vec<(Vec<String>, Vec<String>)>) -> usize {
    input.into_iter().map(solve_line).sum()
}

fn solve_line((code, display): (Vec<String>, Vec<String>)) -> usize {
    let mut code = Code::new(code);

    String::from_iter(display.into_iter().map(|digit| code.determine(digit)))
        .parse::<usize>()
        .unwrap()
}

struct Code {
    known_codes: HashMap<String, char>,
}

impl Code {
    fn new(input_codes: Vec<String>) -> Code {
        let code_1 = find_code_with_length(2, &input_codes);
        let code_4 = find_code_with_length(4, &input_codes);

        let mut of_length_5 = find_codes_with_length(5, &input_codes);
        let mut of_length_6 = find_codes_with_length(6, &input_codes);

        let code_9 = split_off_code_containing(&mut of_length_6, &code_4);
        let code_0 = split_off_code_containing(&mut of_length_6, &code_1);
        let code_6 = of_length_6.pop().unwrap();

        let code_3 = split_off_code_containing(&mut of_length_5, &code_1);
        let code_5 = split_off_code_that_contains(&mut of_length_5, &code_9);
        let code_2 = of_length_5.pop().unwrap();

        let mut known_codes = HashMap::new();
        known_codes.insert(code_1, '1');
        known_codes.insert(code_2, '2');
        known_codes.insert(code_3, '3');
        known_codes.insert(code_4, '4');
        known_codes.insert(code_5, '5');
        known_codes.insert(code_6, '6');
        known_codes.insert(find_code_with_length(3, &input_codes), '7');
        known_codes.insert(find_code_with_length(7, &input_codes), '8');
        known_codes.insert(code_9, '9');
        known_codes.insert(code_0, '0');

        Code { known_codes }
    }

    fn determine(&mut self, code: String) -> char {
        self.known_codes[&sort_chars(code)]
    }
}

fn sort_chars(string: String) -> String {
    let mut chars = string.chars().collect::<Vec<char>>();

    chars.sort();

    String::from_iter(chars.iter())
}

fn find_code_with_length(length: usize, input_codes: &[String]) -> String {
    input_codes
        .iter()
        .find(|code| code.len() == length)
        .cloned()
        .map(sort_chars)
        .unwrap()
}

fn find_codes_with_length(length: usize, input_codes: &[String]) -> Vec<String> {
    input_codes
        .iter()
        .filter(|code| code.len() == length)
        .cloned()
        .map(sort_chars)
        .collect()
}

fn split_off_code_containing(input_codes: &mut Vec<String>, code: &str) -> String {
    let (index, found) = input_codes
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, c)| c.contains_code(&code))
        .unwrap();
    input_codes.remove(index);
    found
}

fn split_off_code_that_contains(input_codes: &mut Vec<String>, code: &str) -> String {
    let (index, found) = input_codes
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, c)| code.contains_code(&c))
        .unwrap();
    input_codes.remove(index);
    found
}

trait ContainsCode {
    fn contains_code(&self, other: &str) -> bool;
}

impl ContainsCode for String {
    fn contains_code(&self, other: &str) -> bool {
        let chars = self.chars().collect::<Vec<char>>();
        for other_char in other.chars() {
            if !chars.contains(&other_char) {
                return false;
            }
        }
        return true;
    }
}

impl ContainsCode for &str {
    fn contains_code(&self, other: &str) -> bool {
        let chars = self.chars().collect::<Vec<char>>();
        for other_char in other.chars() {
            if !chars.contains(&other_char) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    #[test]
    fn test_read_file() {
        let lines = read_file("src/day8/input.txt");

        let (first_code, first_display) = lines.first().unwrap();
        assert_eq!(Some(&"bgcfda".to_string()), first_code.first());
        assert_eq!(Some(&"ae".to_string()), first_display.first());

        let (last_code, last_display) = lines.last().unwrap();
        assert_eq!(Some(&"fcedba".to_string()), last_code.last());
        assert_eq!(Some(&"bg".to_string()), last_display.last());
    }

    #[test]
    fn test_count() {
        let input = read_file("src/day8/test_input.txt");

        let result = count_1_4_7_8(input);

        assert_eq!(26, result);
    }

    #[test]
    fn test_sum_everything() {
        let input = read_file("src/day8/test_input.txt");

        let expected = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

        let line_by_line_solved = input
            .clone()
            .into_iter()
            .map(solve_line)
            .collect::<Vec<usize>>();
        assert_eq!(expected, line_by_line_solved);

        let result = sum_display_values(input);

        assert_eq!(61229, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(369, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(1031553, get_answer_2());
    }
}
