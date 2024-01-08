use std::collections::HashSet;

use rand::Rng;

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

    fn set_solved_node(&mut self, row: usize, col: usize, digit: u8) {
        self.board[row][col] = Digit::from_u8(digit);
    }

    fn get_used_digits(&self, row: usize, col: usize) -> HashSet<u8> {
        let mut collected_vals: HashSet<u8> = HashSet::new();

        for x in 0..9 {
            collected_vals.insert(self.board[x][col] as u8);
            collected_vals.insert(self.board[row][x] as u8);
        }

        collected_vals
    }

    fn is_unit_valid(unit: &HashSet<u8>) -> bool {
        !unit.is_superset(&HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }

    fn generate_solved_grid(&mut self) -> Grid {
        let mut rng = rand::thread_rng();
        let mut valid = false;

        while !valid {
            for row in 0..9 {
                for col in 0..9 {
                    let used_vals = self.get_used_digits(row, col);
                    valid = Sudoku::is_unit_valid(&used_vals);

                    if !valid {
                        self.reset_solved();
                        break;
                    }

                    let mut rand_val = rng.gen_range(1..10);

                    while used_vals.contains(&rand_val) {
                        rand_val = rng.gen_range(1..10);
                    }

                    self.set_solved_node(row, col, rand_val);
                }

                if !valid {
                    self.reset_solved();
                    break;
                }
            }
        }

        self.get_solved()
    }

    pub fn print_board(&self) {
        println!();

        for row in 0..9 {
            for col in 0..9 {
                print!("{:?}  ", self.board[row][col] as u8);
            }

            println!();
        }
    }
}
