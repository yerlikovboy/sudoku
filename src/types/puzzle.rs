use std::collections::HashSet;
use std::str;

use crate::Cell;

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
pub struct Puzzle {
    _grid: Vec<u8>,
    _clues_indices: HashSet<u8>,
}

impl Puzzle {
    pub fn new(grid: &[u8]) -> Puzzle {
        Puzzle {
            _grid: grid.to_vec(),
            _clues_indices: (0..81)
                .filter(|x| grid[*x as usize] != 0)
                .collect::<HashSet<u8>>(),
        }
    }

    pub fn grid_as_ref(&self) -> &[u8] {
        &self._grid
    }

    fn check_peers(&self, c: &Cell) -> bool {
        if !c.value.is_none() {
            for idx in c.peers() {
                if self._grid[idx] == c.value.unwrap() {
                    return true;
                }
            }
        }
        false
    }

    pub fn block_for_cell(&self, c: &Cell) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(9);

        let start_offset = (c.row / 3) * 3;
        let start_col = (c.column / 3) * 3;

        for i in 0..3 {
            let row_offset = (start_offset + i) * 9;
            for j in 0..3 {
                let col_idx = start_col + j;
                let idx = row_offset + col_idx;
                let val = self._grid[idx as usize];
                if val != 0 {
                    v.push(val);
                }
            }
        }
        v
    }

    fn _get_index_value(m: &Cell) -> u8 {
        (m.row * 9) + m.column
    }

    pub fn update_cell(&mut self, m: &Cell) -> Result<Cell, &str> {
        let idx = Puzzle::_get_index_value(m);
        if self._clues_indices.contains(&idx) {
            return Err("cannot update initial board value");
        }

        if self.check_peers(m) {
            return Err("cell has a peer which already contains value");
        }

        /*
        if self._check_row_col(m) == false {
            return Err("row/column already has value");
        }
        if self._check_box(m) == false {
            return Err("value already exists in box");
        }
        */

        let updated_cell = Cell {
            previous_value: Some(self._grid[idx as usize]),
            ..*m
        };
        self._grid[idx as usize] = m.value.unwrap_or(0);
        Ok(updated_cell)
    }

    pub fn print_console(&self) {
        println!("Puzzle");
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
