use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

fn fluent_read(file_name: &str) -> io::Result<Vec<u8>> {
    println!("fluent read");
    let f = File::open(file_name)?;

    let vals: Vec<u8> = f
        .bytes()
        .map(|x| x.unwrap())
        .filter(|x| x.is_ascii_digit())
        .map(|x| x - 48)
        .collect();

    println!("# values read: {}", vals.len());
    Ok(vals)
}

fn main() {
    let fname = "/Users/ali/dev/rust/projects/sudoku/board.txt";
    println!("{:?}", fluent_read(fname).unwrap());
}
