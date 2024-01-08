mod sudoku;

use sudoku::Sudoku;

fn main() {
    let sudoku = Sudoku::new();
    sudoku.print_board();
}
