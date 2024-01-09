use std::collections::HashSet;

use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy)]
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
    //state: Grid,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut board = Sudoku {
            board: [[Digit::Zero; 9]; 9],
        };

        board.generate_solved_grid();
        board
    }

    fn get_solved(&self) -> Grid {
        self.board
    }

    /*fn set_solved(&mut self, grid: Grid) {
        self.solved = grid;
    }*/

    fn reset_solved(&mut self) {
        self.board = [[Digit::Zero; 9]; 9];
    }

    /*fn get_current(&self) -> GridCurrent {
        self.current
    }

    fn get_solved_node(&self, x: usize, y: usize) -> Digit {
        self.solved[x][y]
    }

    fn get_current_node(&self, x: usize, y: usize) -> Digit {
        self.current[x][y]
    }*/

    fn set_board_node(&mut self, row: usize, col: usize, digit: u8) {
        self.board[row][col] = Digit::from_u8(digit);
    }

    fn get_used_digits(&self, row: usize, col: usize) -> HashSet<u8> {
        let mut used_nodes: HashSet<u8> = HashSet::new();

        for i in 0..9 {
            used_nodes.insert(self.board[i][col] as u8);
            used_nodes.insert(self.board[row][i] as u8);
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

    fn generate_solved_grid(&mut self) -> Grid {
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

        self.get_solved()
    }

    pub fn print_board(&self) {
        let mut board_str = "\n".to_string();

        for row in 0..9 {
            for col in 0..9 {
                board_str = format!("{}{}  ", board_str, self.board[row][col] as u8);
            }

            board_str = format!("{}\n", board_str);
        }

        print!("{}", board_str);
    }
}
