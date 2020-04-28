use sudoku::console::utils;
use sudoku::game::cell;
use sudoku::game::puzzle;

fn dump(c: &cell::Cell) {
    println!("cell: {}", c);
    let mut v: [u8; 81] = [0; 81];
    c.peers().iter().for_each(|x| v[*x as usize] = 1);
    let p = puzzle::Puzzle::new(&v[..]);
    utils::print_console(&p);
}

fn main() {
    let cells: [cell::Cell; 12] = [
        cell::Cell {
            row: 1,
            column: 2,
            ..Default::default()
        },
        cell::Cell {
            row: 2,
            column: 3,
            ..Default::default()
        },
        cell::Cell {
            row: 0,
            column: 7,
            ..Default::default()
        },
        cell::Cell {
            row: 3,
            column: 3,
            ..Default::default()
        },
        cell::Cell {
            row: 4,
            column: 1,
            ..Default::default()
        },
        cell::Cell {
            row: 4,
            column: 4,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 2,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 5,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 8,
            ..Default::default()
        },
        cell::Cell {
            row: 6,
            column: 6,
            ..Default::default()
        },
        cell::Cell {
            row: 7,
            column: 7,
            ..Default::default()
        },
        cell::Cell {
            row: 8,
            column: 8,
            ..Default::default()
        },
    ];

    for cell in &cells[..] {
        dump(&cell);
    }
}
