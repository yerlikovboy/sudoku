use std::env;

use sudoku::console::cli;
use sudoku::console::utils;

fn usage() {
    println!("usage: sudoku <filename>\n\twhere filename denotes file with sudoku puzzle.\n\nExample:\n\tsudoku puzzles/easy-board-1.txt");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        return;
    }

    let file_name = &args[1];
    let mut board = utils::from_file(file_name).unwrap();
    cli::play(&mut board);

    println!("Finished!");
}
