use std::fs::File;
use std::io::Read;

use crate::game::cell::Cell;
use crate::game::puzzle::Puzzle;

pub fn print_puzzle(p: &Puzzle) {
    print_puzzle_cells(p.grid());
}

pub fn print_puzzle_cells(p: &[Cell]) {
    println!("Puzzle: Cells");
    for r in 0..9 {
        if r % 3 == 0 {
            println!("-------------------------");
        }
        for c in 0..9 {
            if c % 3 == 0 {
                print!("| ");
            }
            let cell = &p[(r * 9) + c];
            let v = cell.value();
            if v.is_none() {
                print!(". ");
            } else {
                let n = v.unwrap();
                if cell.has_conflicts() {
                    print!("\x1B[38;5;160m{}\x1B[0m ", n);
                } else if cell.is_clue() {
                    print!("\x1B[38;5;111m{}\x1B[0m ", n);
                } else {
                    print!("\x1B[38;5;221m{}\x1B[0m ", n);
                }
            }
        }
        print!("|\n");
    }
    println!("-------------------------");
}

pub fn print_console(grid: &[u8], row_dim: usize, col_dim: usize) {
    println!("Puzzle");
    for r in 0..row_dim {
        // TODO: the three needs to be derived from dimension info.
        if r % 3 == 0 {
            println!("-------------------------");
        }
        for c in 0..col_dim {
            // TODO: the three needs to be derived from dimension info.
            if c % 3 == 0 {
                print!("| ");
            }
            let v = grid[(r * row_dim) + c];
            if v == 0 {
                print!(". ");
            } else {
                print!("{} ", v);
            }
        }
        print!("|\n");
    }
    println!("-------------------------");
}

pub fn from_file(file_name: &str) -> std::io::Result<Puzzle> {
    let file = File::open(file_name)?;

    let grid = file
        .bytes()
        .map(|x| x.unwrap())
        .filter(|x| x.is_ascii_digit())
        .map(|x| x - 48)
        .collect::<Vec<u8>>();

    assert_eq!(grid.len(), 81);
    Ok(Puzzle::new(grid.as_slice()))
}

pub fn user_move(row: usize, column: usize, value: u8) -> Result<Cell, String> {
    if row < 1 || row > 9 {
        return Err(String::from("row must be in the range 1..9"));
    }

    if column < 1 || column > 9 {
        return Err(String::from("column must be in the range 1..9"));
    }
    if value > 9 {
        return Err(String::from("new value must be between 0 and 9"));
    }

    Ok(Cell::user_request(
        row - 1,
        column - 1,
        if value == 0 { None } else { Some(value) },
    ))
}
