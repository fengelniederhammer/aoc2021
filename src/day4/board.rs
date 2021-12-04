use nalgebra::{
    ArrayStorage, Const, MatrixSlice, MatrixSlice5x1, RowVector5, SquareMatrix, U1, U5,
};

#[derive(Debug)]
pub struct BingoBoard {
    matrix: SquareMatrix<(isize, bool), U5, ArrayStorage<(isize, bool), 5, 5>>,
}

impl BingoBoard {
    pub fn builder() -> BoardBuilder {
        BoardBuilder { rows: vec![] }
    }

    pub fn get(&self, row: usize, column: usize) -> (isize, bool) {
        self.matrix[(row, column)]
    }

    pub fn compute_unmarked_score(&self) -> isize {
        self.matrix
            .iter()
            .filter(|(_, marked)| !*marked)
            .map(|(value, _)| value)
            .sum()
    }

    pub fn mark_value(&mut self, value: isize) {
        self.matrix
            .iter_mut()
            .filter(|(stored_value, _)| stored_value == &value)
            .for_each(|(_, marked)| *marked = true);
    }

    pub fn bingo(&self) -> bool {
        let has_completed_column = self.matrix.column_iter().fold(false, |completed, column| {
            completed || is_column_completed(column)
        });
        let has_completed_row = self
            .matrix
            .row_iter()
            .fold(false, |completed, row| completed || is_row_completed(row));

        has_completed_row || has_completed_column
    }
}

fn is_column_completed(column: MatrixSlice5x1<(isize, bool)>) -> bool {
    column.iter().fold(true, |column_completed, (_, marked)| {
        column_completed && *marked
    })
}

fn is_row_completed(row: MatrixSlice<(isize, bool), U1, U5, Const<1>, Const<5>>) -> bool {
    row.iter()
        .fold(true, |row_completed, (_, marked)| row_completed && *marked)
}

pub struct BoardBuilder {
    rows: Vec<RowVector5<(isize, bool)>>,
}

impl BoardBuilder {
    pub fn add_row(&mut self, row_str: &str) {
        let row = row_str
            .split_whitespace()
            .map(|entry| (entry.parse::<isize>().unwrap(), false));
        self.rows.push(RowVector5::from_iterator(row));
    }

    pub fn build(self) -> BingoBoard {
        if self.rows.len() != 5 {
            panic!("Länge war {}", self.rows.len());
        };

        BingoBoard {
            matrix: SquareMatrix::from_rows(&self.rows),
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{RowVector5, SquareMatrix, U5};

    use crate::day4::board::BingoBoard;

    #[test]
    fn board_score() {
        let under_test = BingoBoard {
            matrix: SquareMatrix::from_rows(&vec![
                RowVector5::from_element((1, false)),
                RowVector5::from_element((2, false)),
                RowVector5::from_element((3, false)),
                RowVector5::from_element((4, false)),
                RowVector5::from_element((5, false)),
            ]),
        };

        assert_eq!(75, under_test.compute_unmarked_score());
    }

    #[test]
    fn tick_value() {
        let mut under_test = BingoBoard {
            matrix: SquareMatrix::<_, U5, _>::from_fn(|i, j| ((i * 10 + j) as isize, false)),
        };

        under_test.mark_value(6);

        let is_value_ticked = under_test
            .matrix
            .fold(false, |result, (_, ticked)| result || ticked);
        assert!(!is_value_ticked);

        under_test.mark_value(4);

        assert_eq!((4, true), under_test.get(0, 4));
    }

    #[test]
    fn bingo_vertical() {
        let mut under_test = BingoBoard {
            matrix: SquareMatrix::<_, U5, _>::from_fn(|i, j| ((i * 10 + j) as isize, false)),
        };

        assert!(!under_test.bingo());

        under_test.mark_value(0);

        assert!(!under_test.bingo());

        under_test.mark_value(10);

        assert!(!under_test.bingo());

        under_test.mark_value(20);

        assert!(!under_test.bingo());

        under_test.mark_value(30);

        assert!(!under_test.bingo());

        under_test.mark_value(40);

        assert!(under_test.bingo());
    }

    #[test]
    fn bingo_horizontal() {
        let mut under_test = BingoBoard {
            matrix: SquareMatrix::<_, U5, _>::from_fn(|i, j| ((i * 10 + j) as isize, false)),
        };

        assert!(!under_test.bingo());

        under_test.mark_value(20);

        assert!(!under_test.bingo());

        under_test.mark_value(21);

        assert!(!under_test.bingo());

        under_test.mark_value(22);

        assert!(!under_test.bingo());

        under_test.mark_value(23);

        assert!(!under_test.bingo());

        under_test.mark_value(24);

        assert!(under_test.bingo());
    }
}
