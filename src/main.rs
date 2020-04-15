use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

// lessons learned:
//
// numeric values have to be converted to usize if they are to be used
// to retrieve a value from an array index. By definition, the size
// of usize is how many bytes it takes to reference any location in
// memory.
//
// Did not need to use this but its worth noting ...
// Note that there is a cast in the filter statement ...
//  let s: HashSet<u8> = (0..81).filter(|x| v_arr[*x as usize] != 0).collect();
// Why do we need to cast the number as usize? Because Rust is trying to determine the type for number
// from the range 0..81 and it needs usize to index the array and u8 for the values in the HashSet. If
// the value x in the filter method is not cast as usize, the compiler will complain that it cannot
// create a HashSet of u8 from usize values. This is because it tries to satisfy the requirement for
// array indices to be usize and sets the type as usize. The collect method receives values as usize
// and doesnt know what to do with them.
//
// A more explicit approach is to use map to convert the values from usize to u8 after filter:
// let s: HashSet<u8> = (0..81)
//     .filter(|x| v_arr[*x] != 0)
//     .map(|x| x as u8)
//     .collect();
//
// Result error type cannot be a string slice &str unless at least one of the parameters is a
// reference.
//
// These will both trigger a "missing lifetime specifier" error:
// fn why(v: u8) -> Result<(), &str> {
//    Ok()
// }
//
// This is because the parameter is of type Copy and will not be valid once the function returns.
// Because of this, the compiler has no way of knowing what to set the lifespan of &str.
//
// This will cause a compilation error of "missing lifetime specifier"
// fn also_why() -> Result<(), &str> {
//     Ok(())
// }
// Again, no way to resolve the lifespan of &str from this.

struct Board {
    _vals: [u8; 81],
    _start_pos: HashSet<u8>,
}

impl Board {
    fn _init() -> Board {
        Board {
            _vals: [0; 81],
            _start_pos: HashSet::new(),
        }
    }

    // return true if ok to insert, false otherwise.
    fn _check_row_col(&self, m: &Move) -> bool {
        for i in 0..9 {
            // check row
            let row_val = self._vals[((m.row) * 9 + i) as usize];
            if row_val == m.value {
                // if the new value is same as old value, its ok ...
                return m.column == i;
            }
            // check column
            let col_val = self._vals[((i * 9) + m.column) as usize];
            if col_val == m.value {
                return m.row == i;
            }
        }
        true
    }

    fn _check_box(&self, m: &Move) -> bool {
        // find the closest multiple of three
        // less than or equal to x
        let start_offset = (m.row / 3) * 3;
        let start_col = (m.column / 3) * 3;

        for i in 0..3 {
            let row_offset = (start_offset + i) * 9;
            for j in 0..3 {
                let col_idx = start_col + j;
                let idx = row_offset + col_idx;

                if self._vals[idx as usize] == m.value {
                    println!(
                        "value already exists at x: {}, y: {}, value: {},exising: {}",
                        i,
                        j,
                        m.value,
                        self._vals[(i + j) as usize]
                    );
                    return false;
                }
            }
        }
        true
    }

    fn _get_index_value(m: &Move) -> u8 {
        (m.row * 9) + m.column
    }

    fn set_value(&mut self, m: &Move) -> Result<(), &str> {
        let idx = Board::_get_index_value(m);
        if self._start_pos.contains(&idx) {
            return Err("cannot update initial board value");
        }

        if self._check_row_col(m) == false {
            return Err("row/column already has value");
        }
        if self._check_box(m) == false {
            return Err("value already exists in box");
        }
        println!(
            "move: row:{}, column:{}, new value: {}, previous value:{}",
            m.row, m.column, m.value, self._vals[idx as usize]
        );
        self._vals[idx as usize] = m.value;
        Ok(())
    }

    fn load_from_file(file_name: &str) -> std::io::Result<Board> {
        let zero = b'0';

        let file = File::open(file_name)?;

        let mut count: u8 = 0;
        let mut v_arr: [u8; 81] = [0; 81];
        let mut spots: HashSet<u8> = HashSet::new();

        for curr_byte in file.bytes() {
            if count > 80 {
                println!("warning: file has more than 81 entries");
                break;
            }
            let v = curr_byte.unwrap();
            // skip spaces ...
            if v.is_ascii_digit() {
                v_arr[count as usize] = v - 48;
                // if the value is not a zero, mark the location as init values
                if v != zero {
                    spots.insert(count);
                }
                count = count + 1;
            }
        }
        // make sure we read in 81 values
        assert_eq!(count, 81);

        Ok(Board {
            _vals: v_arr,
            _start_pos: spots,
        })
    }

    fn print_console(&self) {
        println!("Board");
        for i in 0..9 {
            if i % 3 == 0 {
                println!("-------------------------");
            }
            for j in 0..9 {
                if j % 3 == 0 {
                    print!("| ");
                }
                let v = self._vals[(i * 9) + j];
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

    fn check_if_finished(&self) -> bool {
        for i in 0..81 {
            if self._vals[i] == 0 {
                return false;
            }
        }
        true
    }
}

fn play(board: &mut Board) {
    loop {
        board.print_console();
        println!("Please enter your next move (row column value) or Ctrl-C to quit: ");

        let mut raw_input = String::new();
        io::stdin()
            .read_line(&mut raw_input)
            .expect("error reading input from user");

        let next_move: Vec<u8> = raw_input
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let m = Move::new(next_move[0], next_move[1], next_move[2]).unwrap();

        match board.set_value(&m) {
            Ok(_) => (),
            Err(msg) => println!(
                "unable to make move row:{}, col:{}, new value:{} -> {} ",
                next_move[0], next_move[1], next_move[2], msg
            ),
        }
        if board.check_if_finished() {
            println!("Congrats! You've won!");
            break;
        }
    }
}

struct Move {
    row: u8,
    column: u8,
    value: u8,
}

impl Move {
    fn new(row: u8, column: u8, value: u8) -> Result<Move, String> {
        if row < 1 || row > 9 {
            return Err(String::from("row must be in the range 1..9"));
        }

        if column < 1 || column > 9 {
            return Err(String::from("column must be in the range 1..9"));
        }
        if value > 9 {
            return Err(String::from("new value must be between 0 and 9"));
        }

        Ok(Move {
            row: row - 1,
            column: column - 1,
            value: value,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];
    let mut board = Board::load_from_file(file_name).unwrap();
    play(&mut board);
    println!("Finished!");
}
