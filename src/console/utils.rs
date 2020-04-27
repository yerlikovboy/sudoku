use std::fs::File;
use std::io::Read;

use sudoku::Cell;
use sudoku::Puzzle;

pub fn print_console(p: &Puzzle) {
    let _grid = p.grid_as_ref();
    println!("Puzzle");
    for i in 0..9 {
        if i % 3 == 0 {
            println!("-------------------------");
        }
        for j in 0..9 {
            if j % 3 == 0 {
                print!("| ");
            }
            let v = _grid[(i * 9) + j];
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

pub fn user_move(row: u8, column: u8, value: u8) -> Result<Cell, String> {
    if row < 1 || row > 9 {
        return Err(String::from("row must be in the range 1..9"));
    }

    if column < 1 || column > 9 {
        return Err(String::from("column must be in the range 1..9"));
    }
    if value > 9 {
        return Err(String::from("new value must be between 0 and 9"));
    }

    Ok(Cell {
        row: row - 1,
        column: column - 1,
        value: if value == 0 { None } else { Some(value) },
        previous_value: None,
    })
}
