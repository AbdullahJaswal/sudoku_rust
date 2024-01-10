use std::collections::HashSet;

use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Digit {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn from_u8(value: u8) -> Digit {
        match value {
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => Digit::Zero,
        }
    }
}

type Grid = [[Digit; 9]; 9];

#[derive(Debug)]
pub struct Sudoku {
    board: Grid,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut board = Sudoku {
            board: [[Digit::Zero; 9]; 9],
        };

        board.generate_solved_grid();
        board
    }

    fn reset_solved(&mut self) {
        self.board = [[Digit::Zero; 9]; 9];
    }

    fn set_board_node(&mut self, row: usize, col: usize, digit: u8) {
        self.board[row][col] = Digit::from_u8(digit);
    }

    fn get_used_digits(&self, row: usize, col: usize) -> HashSet<u8> {
        let mut used_nodes: HashSet<u8> = HashSet::new();

        for i in 0..9 {
            // nodes in row
            if self.board[row][i] != Digit::Zero {
                used_nodes.insert(self.board[row][i] as u8);
            }

            // nodes in column
            if self.board[i][col] != Digit::Zero {
                used_nodes.insert(self.board[i][col] as u8);
            }

            if used_nodes.len() == 9 {
                return used_nodes;
            }
        }

        // nodes in grid unit
        let grid_row_start = (row / 3) * 3;
        let grid_col_start = (col / 3) * 3;

        for r in grid_row_start..grid_row_start + 3 {
            for c in grid_col_start..grid_col_start + 3 {
                used_nodes.insert(self.board[r][c] as u8);

                if used_nodes.len() == 9 {
                    return used_nodes;
                }
            }
        }

        used_nodes
    }

    fn get_unused_digits(used_nodes: HashSet<u8>) -> Vec<u8> {
        let all_nodes: HashSet<u8> = (1..=9).collect();

        all_nodes.difference(&used_nodes).copied().collect()
    }

    fn is_unit_valid(unit: &HashSet<u8>) -> bool {
        !unit.is_superset(&(1..=9).collect())
    }

    fn generate_solved_grid(&mut self) {
        let mut rng = rand::thread_rng();

        'outer: loop {
            for row in 0..9 {
                for col in 0..9 {
                    let used_nodes = self.get_used_digits(row, col);
                    if !Sudoku::is_unit_valid(&used_nodes) {
                        self.reset_solved();
                        continue 'outer;
                    }

                    let unused_nodes = Sudoku::get_unused_digits(used_nodes);
                    let node = unused_nodes.choose(&mut rng).unwrap();

                    self.set_board_node(row, col, *node);
                }
            }

            break;
        }
    }

    pub fn print_board(&self) {
        let mut board_str = "\n".to_string();

        for row in 0..9 {
            for col in 0..9 {
                board_str = format!("{}{} ", board_str, self.board[row][col] as u8);

                if col == 2 || col == 5 {
                    board_str = format!("{}❘ ", board_str)
                }
            }

            board_str = format!("{}\n", board_str);

            if row == 2 || row == 5 {
                board_str = format!("{}― ― ― ― ― ― ― ― ― ― ―\n", board_str)
            }
        }

        print!("{}", board_str);
    }
}
