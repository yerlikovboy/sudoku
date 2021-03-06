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

use std::str;

use crate::game::cell::Cell;

// classic 9x9 sudoku
pub struct Puzzle {
    grid: Vec<Cell>,
}

impl Puzzle {
    pub fn new(grid: &[u8]) -> Puzzle {
        let mut v: Vec<Cell> = Vec::with_capacity(81);

        for i in 0..81 {
            let clue = grid[i];
            if clue == 0 {
                v.push(Cell::new(i, clue));
            } else {
                v.push(Cell::new_clue(i, grid[i]));
            }
        }

        Puzzle { grid: v }
    }

    #[inline]
    pub fn grid(&self) -> &[Cell] {
        self.grid.as_slice()
    }

    pub fn update_cell(&mut self, m: &Cell) -> Result<(), &str> {
        if self.grid[m.idx()].is_clue() {
            return Err("cell holds clue value");
        }

        let current_value = self.grid[m.idx()].value().unwrap_or(0);
        let new_value = m.value().unwrap_or(0);

        if current_value == new_value {
            return Ok(());
        }
        let mut conflict_count: u8 = 0;

        for peer_idx in m.peers() {
            let peer = &mut self.grid[peer_idx];
            let peer_value = peer.value().unwrap_or(0);

            if current_value != 0 && peer_value == current_value {
                peer.conflict_down();
            }
            if new_value != 0 && peer_value == new_value {
                peer.incr_conflict();
                conflict_count += 1;
            }
        }

        // why is this down here and not above? Because the line where the peer cell
        // is referenced requires a mutable reference from the same vector as where
        // curr resides.
        let curr = &mut self.grid[m.idx()];
        curr.set_value(new_value);
        curr.set_conflicts(conflict_count);

        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        self.grid.iter().all(|x| x.value().is_some())
    }
}
