use std::fs::read_to_string;

pub fn get_answer_1() -> usize {
    get_error_score(read_file("src/day10/input.txt"))
}

pub fn get_answer_2() -> usize {
    find_middle_completion_score(read_file("src/day10/input.txt"))
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let file_content = read_to_string(path).expect("hoppla");

    file_content
        .lines()
        .map(|str| str.chars().collect())
        .collect()
}

fn get_error_score(input: Vec<Vec<char>>) -> usize {
    input.into_iter().map(compute_line_score).sum()
}

fn compute_line_score(line: Vec<char>) -> usize {
    match compute_line_state(line) {
        Line::Corrupted(score) => score,
        Line::Complete | Line::Incomplete(_) => 0,
    }
}

fn find_middle_completion_score(input: Vec<Vec<char>>) -> usize {
    let mut scores = input.into_iter().filter_map(do_it).collect::<Vec<usize>>();

    scores.sort();

    scores[scores.len() / 2]
}

fn do_it(line: Vec<char>) -> Option<usize> {
    match compute_line_state(line) {
        Line::Corrupted(_) | Line::Complete => None,
        Line::Incomplete(missing_brackets) => Some(
            missing_brackets
                .into_iter()
                .rev()
                .fold(0, |score, bracket| 5 * score + bracket.completion_score()),
        ),
    }
}

fn compute_line_state(line: Vec<char>) -> Line {
    let mut bracket_stack = vec![];

    for bracket in line {
        match bracket {
            '(' => bracket_stack.push(Bracket::Round),
            '[' => bracket_stack.push(Bracket::Square),
            '{' => bracket_stack.push(Bracket::Curly),
            '<' => bracket_stack.push(Bracket::Pointy),
            ')' => match bracket_stack.pop() {
                Some(Bracket::Round) => {}
                _ => return Line::Corrupted(3),
            },
            ']' => match bracket_stack.pop() {
                Some(Bracket::Square) => {}
                _ => return Line::Corrupted(57),
            },
            '}' => match bracket_stack.pop() {
                Some(Bracket::Curly) => {}
                _ => return Line::Corrupted(1197),
            },
            '>' => match bracket_stack.pop() {
                Some(Bracket::Pointy) => {}
                _ => return Line::Corrupted(25137),
            },
            unexpected => panic!("{} should not be here", unexpected),
        }
    }

    match bracket_stack.len() {
        0 => Line::Complete,
        _ => Line::Incomplete(bracket_stack),
    }
}

enum Line {
    Corrupted(usize),
    Complete,
    Incomplete(Vec<Bracket>),
}

enum Bracket {
    Round,
    Square,
    Curly,
    Pointy,
}

impl Bracket {
    fn completion_score(&self) -> usize {
        match self {
            Bracket::Round => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Pointy => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn test_read_file() {
        let input = read_file("src/day10/input.txt");

        assert_eq!('(', input[0][0]);
        assert_eq!(&'(', input.last().unwrap().last().unwrap());
    }

    #[test]
    fn test_score() {
        let input = read_file("src/day10/test_input.txt");

        let result = get_error_score(input);

        assert_eq!(26397, result);
    }

    #[test]
    fn test_score_completions() {
        let input = read_file("src/day10/test_input.txt");

        let result = find_middle_completion_score(input);

        assert_eq!(288957, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(389589, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(1190420163, get_answer_2());
    }
}
