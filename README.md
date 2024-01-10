# Sudoku Puzzle Generator

The Sudoku generator, developed in Rust, uses a randomized backtracking algorithm to fill the grid. For each cell, it first checks which digits are already used in the same row, column, and 3x3 grid, and then selects a random digit from the unused ones. If it finds that a cell cannot be filled with any unused digit (no valid number can be placed in that cell without breaking Sudoku rules), the algorithm resets the entire grid and starts over. This process of trial and error continues until the algorithm successfully fills the entire grid with valid numbers in every cell. The algorithm goes through multiple iterations until it creates a valid grid and is efficient enough to generate a puzzle within a second.