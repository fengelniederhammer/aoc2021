use std::fs::read_to_string;

use crate::day4::board::{BingoBoard, BoardBuilder};

mod board;

pub fn get_answer_1() -> isize {
    play(read_file("src/day4/input.txt"))
}

pub fn get_answer_2() -> isize {
    play_badly(read_file("src/day4/input.txt"))
}

fn read_file(path: &str) -> (Vec<isize>, Vec<BingoBoard>) {
    let file_content = read_to_string(path).expect("hoppla");

    let lines: Vec<&str> = file_content.lines().collect();

    let numbers = lines
        .first()
        .unwrap()
        .split(",")
        .map(|number| number.parse::<isize>().unwrap())
        .collect();

    let (mut bingo_boards, board_builder) = lines.iter().skip(2).fold(
        (vec![], BingoBoard::builder()),
        |(bingo_boards, board_builder), line| add_line_to_boards(bingo_boards, board_builder, line),
    );

    bingo_boards.push(board_builder.build());

    (numbers, bingo_boards)
}

fn add_line_to_boards(
    mut bingo_boards: Vec<BingoBoard>,
    mut board_builder: BoardBuilder,
    line: &str,
) -> (Vec<BingoBoard>, BoardBuilder) {
    match line.len() {
        0 => {
            bingo_boards.push(board_builder.build());
            (bingo_boards, BingoBoard::builder())
        }
        _ => {
            board_builder.add_row(line);
            (bingo_boards, board_builder)
        }
    }
}

fn play((numbers, mut boards): (Vec<isize>, Vec<BingoBoard>)) -> isize {
    for number in numbers {
        for board in &mut boards {
            board.mark_value(number);
            if board.bingo() {
                return number * board.compute_unmarked_score();
            }
        }
    }
    panic!("ohoh!")
}

fn play_badly((numbers, mut boards): (Vec<isize>, Vec<BingoBoard>)) -> isize {
    let number_of_boards = boards.len();
    let mut winning_boards = vec![];

    for number in numbers {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if winning_boards.contains(&board_index) {
                continue;
            }

            board.mark_value(number);
            if board.bingo() {
                if winning_boards.len() < number_of_boards - 1 {
                    winning_boards.push(board_index);
                } else {
                    return number * board.compute_unmarked_score();
                }
            }
        }
    }
    panic!("ohoh!")
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    fn test_read_file() {
        let (numbers, bingo_boards) = read_file("src/day4/input.txt");

        assert_eq!(Some(&90), numbers.first());
        assert_eq!(Some(&65), numbers.last());

        assert_eq!((76, false), bingo_boards.first().unwrap().get(0, 0));
        assert_eq!((60, false), bingo_boards.last().unwrap().get(4, 4));
    }

    #[test]
    fn test_input() {
        let result = play(read_file("src/day4/test_input.txt"));

        assert_eq!(4512, result);
    }

    #[test]
    fn test_input_play_badly() {
        let result = play_badly(read_file("src/day4/test_input.txt"));

        assert_eq!(1924, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(8136, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(12738, get_answer_2());
    }
}
