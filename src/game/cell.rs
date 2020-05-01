use std::fmt;

use crate::game::puzzle::Puzzle;

#[derive(Default)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub idx: usize,
}

impl Cell {
    pub fn new(row: usize, column: usize) -> Cell {
        Cell {
            row,
            column,
            idx: Cell::to_grid_idx(row, column),
        }
    }

    pub fn from_grid_idx(idx: usize) -> Cell {
        Cell {
            row: idx / 9,
            column: idx % 9,
            idx,
        }
    }

    #[inline]
    pub fn to_grid_idx(row: usize, column: usize) -> usize {
        (row * 9) + column
    }

    #[inline]
    pub fn as_grid_idx(&self) -> usize {
        self.idx
    }

    pub fn value(&self, p: &Puzzle) -> Option<u8> {
        let v = p.grid_as_ref()[self.as_grid_idx()];
        if v != 0 {
            return Some(v);
        }
        None
    }

    pub fn peers(&self) -> Vec<usize> {
        let mut _mapper: [bool; 81] = [false; 81];

        //rows
        let row_idx: usize = (self.idx / 9) * 9;
        let row_end: usize = row_idx + 9;
        (row_idx..row_end)
            .filter(|x| *x != self.idx)
            .for_each(|x| _mapper[x] = true);

        // cols
        let col_offset: usize = self.idx % 9;
        (0..9)
            .map(|x| (x as usize * 9) + col_offset)
            .filter(|x| *x != self.idx)
            .for_each(|x| _mapper[x] = true);

        // block
        let block_x: usize = (self.row as usize / 3) * 3;
        let block_y: usize = (self.column as usize / 3) * 3;
        for i in 0..3 {
            let row_offset = (block_x + i) * 9;
            for j in 0..3 {
                let col_idx = block_y + j;
                let idx = row_offset + col_idx;
                if self.idx != idx {
                    _mapper[idx] = true;
                }
            }
        }

        // TODO: since there is always exactly 20 peers,
        // there is no need to go through all 81 cells unless
        // one of the peers is the last cell.
        (0..81)
            .filter(|x| _mapper[*x as usize] == true)
            .collect::<Vec<usize>>()
    }

    pub fn block_idx(&self) -> Vec<usize> {
        let mut r: Vec<usize> = Vec::with_capacity(8);

        let _self_idx = self.as_grid_idx();
        let block_x: usize = (self.row as usize / 3) * 3;
        let block_y: usize = (self.column as usize / 3) * 3;
        for i in 0..3 {
            let row_offset = (block_x + i) * 9;
            for j in 0..3 {
                let col_idx = block_y + j;
                let idx = row_offset + col_idx;
                if _self_idx != idx {
                    r.push(idx);
                }
            }
        }
        r
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("[row: {}, column: {}]", self.row, self.column,))
    }
}
