use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str;
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

// classic 9x9 sudoku
pub struct Board {
    _grid: [u8; 81],
    _clues_indices: HashSet<u8>,
}

impl Board {
    pub fn init() -> Board {
        Board {
            _grid: [0; 81],
            _clues_indices: HashSet::new(),
        }
    }

    pub fn grid(&self) -> Vec<u8> {
        let v: Vec<u8> = self._grid.iter().map(|x| *x).collect();
        v
    }
    // return true if ok to insert, false otherwise.
    fn _check_row_col(&self, m: &Cell) -> bool {
        for i in 0..9 {
            // check row
            let row_val = self._grid[((m.row) * 9 + i) as usize];
            if row_val == m.value {
                // if the new value is same as old value, its ok ...
                return m.column == i;
            }
            // check column
            let col_val = self._grid[((i * 9) + m.column) as usize];
            if col_val == m.value {
                return m.row == i;
            }
        }
        true
    }

    fn _check_box(&self, m: &Cell) -> bool {
        // find the closest multiple of three
        // less than or equal to x
        let start_offset = (m.row / 3) * 3;
        let start_col = (m.column / 3) * 3;

        for i in 0..3 {
            let row_offset = (start_offset + i) * 9;
            for j in 0..3 {
                let col_idx = start_col + j;
                let idx = row_offset + col_idx;

                if self._grid[idx as usize] == m.value {
                    /*
                    println!(
                        "value already exists at x: {}, y: {}, value: {},exising: {}",
                        i,
                        j,
                        m.value,
                        self._grid[(i + j) as usize]
                    );
                    */
                    return false;
                }
            }
        }
        true
    }

    fn _get_index_value(m: &Cell) -> u8 {
        (m.row * 9) + m.column
    }

    pub fn update_cell(&mut self, m: &Cell) -> Result<Cell, &str> {
        let idx = Board::_get_index_value(m);
        if self._clues_indices.contains(&idx) {
            return Err("cannot update initial board value");
        }

        if self._check_row_col(m) == false {
            return Err("row/column already has value");
        }
        if self._check_box(m) == false {
            return Err("value already exists in box");
        }

        let updated_cell = Cell {
            previous_value: Some(self._grid[idx as usize]),
            ..*m
        };
        self._grid[idx as usize] = m.value;
        Ok(updated_cell)
    }

    pub fn load_from_file(file_name: &str) -> std::io::Result<Board> {
        let zero = b'0';

        let file = File::open(file_name)?;

        let mut count: u8 = 0;
        let mut v_arr: [u8; 81] = [0; 81];
        let mut spots: HashSet<u8> = HashSet::new();

        for curr_byte in file.bytes() {
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
            _grid: v_arr,
            _clues_indices: spots,
        })
    }

    pub fn print_console(&self) {
        println!("Board");
        for i in 0..9 {
            if i % 3 == 0 {
                println!("-------------------------");
            }
            for j in 0..9 {
                if j % 3 == 0 {
                    print!("| ");
                }
                let v = self._grid[(i * 9) + j];
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

    pub fn is_completed(&self) -> bool {
        for i in 0..81 {
            if self._grid[i] == 0 {
                return false;
            }
        }
        true
    }
}

pub struct Cell {
    row: u8,
    column: u8,
    value: u8,
    previous_value: Option<u8>,
}

impl Cell {
    pub fn new(row: u8, column: u8, value: u8) -> Result<Cell, String> {
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
            value: value,
            previous_value: Option::None,
        })
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //let previous_value = self.previous_value.unwrap_or(0);
        let previous_value = self
            .previous_value
            .map_or(String::from("none"), |x| x.to_string());

        formatter.write_fmt(format_args!(
            "cell: [ row: {}, column: {}, value: {}, previous_value: {}]",
            self.row, self.column, self.value, previous_value
        ))
    }
}
