use std::env;
use sudoku::Board;

fn usage() {
    println!("Usage:\n\tsudoku <filename>\n\nExample:\n\t sudoku examples/easy-board.txt");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        return;
    }
    let file_name = &args[1];
    let mut board = Board::load_from_file(file_name).unwrap();
    sudoku::play(&mut board);
    println!("Finished!");
}
