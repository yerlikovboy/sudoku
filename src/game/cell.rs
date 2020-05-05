use std::fmt;

#[derive(Default, Clone)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub idx: usize,
    value: Option<u8>,
    is_clue: bool,
    conflicts: u8,
}

#[inline]
fn row_from_idx(idx: usize) -> usize {
    idx / 9
}

#[inline]
fn col_from_idx(idx: usize) -> usize {
    idx % 9
}

#[inline]
fn to_grid_idx(row: usize, column: usize) -> usize {
    (row * 9) + column
}

impl Cell {
    #[inline]
    pub fn idx(&self) -> usize {
        self.idx
    }

    #[inline]
    pub fn value(&self) -> Option<u8> {
        self.value
    }

    #[inline]
    pub fn is_clue(&self) -> bool {
        self.is_clue
    }

    #[inline]
    pub fn as_grid_idx(&self) -> usize {
        self.idx
    }

    pub fn new(idx: usize, value: u8) -> Cell {
        Cell {
            idx,
            row: row_from_idx(idx),
            column: col_from_idx(idx),
            value: if value == 0 { None } else { Some(value) },
            is_clue: false,
            conflicts: 0,
        }
    }

    pub fn user_request(row: usize, column: usize, value: Option<u8>) -> Cell {
        Cell {
            row,
            column,
            idx: to_grid_idx(row, column),
            value,
            is_clue: false,
            conflicts: 0,
        }
    }

    pub fn new_clue(idx: usize, value: u8) -> Cell {
        Cell {
            row: row_from_idx(idx),
            column: col_from_idx(idx),
            idx,
            value: Some(value),
            is_clue: true,
            conflicts: 0,
        }
    }

    pub fn from_grid_idx(idx: usize) -> Cell {
        Cell {
            row: idx / 9,
            column: idx % 9,
            idx,
            value: None,
            is_clue: false,
            conflicts: 0,
        }
    }

    pub fn has_conflicts(&self) -> bool {
        self.conflicts != 0
    }

    pub fn incr_conflict(&mut self) {
        self.conflicts += 1;
    }

    pub fn conflict_down(&mut self) {
        self.conflicts -= 1;
    }

    pub fn set_conflicts(&mut self, c: u8) {
        self.conflicts = c;
    }

    pub fn clear_conflicts(&mut self) {
        self.conflicts = 0;
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = if value == 0 { None } else { Some(value) }
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
