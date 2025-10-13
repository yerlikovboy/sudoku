use std::env;

use sudoku::console::{cli, interactive, utils};

fn usage() {
    println!("usage: sudoku <filename> [--classic]\n\twhere filename denotes file with sudoku puzzle.\n\nOptions:\n\t--classic    Use classic text-based input mode\n\nExample:\n\tsudoku puzzles/easy-board-1.txt\n\tsudoku puzzles/easy-board-1.txt --classic");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        usage();
        return;
    }

    let file_name = &args[1];
    let use_classic = args.len() == 3 && args[2] == "--classic";

    let mut board = utils::from_file(file_name).unwrap();

    if use_classic {
        cli::play(&mut board);
    } else {
        interactive::play(&mut board).unwrap();
    }
}
