use crate::game::cell::Cell;
use std::fmt;

pub struct Move {
    pub n: u32,
    idx: usize,
    value: u8,
}

impl Move {
    pub fn new(n: u32, row: usize, column: usize, value: u8) -> Move {
        Move {
            n,
            idx: Cell::to_grid_idx(row, column),
            value,
        }
    }

    pub fn cell(&self) -> Cell {
        Cell::from_grid_idx(self.idx)
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    #[inline]
    pub fn idx(&self) -> usize {
        self.idx
    }
}
impl fmt::Display for Move {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "n: {}, cell: {}, value: {}",
            self.n,
            Cell::from_grid_idx(self.idx),
            self.value
        ))
    }
}

impl fmt::Display for Update {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "[num: {}, cell: {}, value: {}, prev: {}]",
            self.num,
            Cell::from_grid_idx(self.idx),
            self.new_value,
            self.prev_value
        ))
    }
}
#[derive(Default)]
pub struct Update {
    pub num: u32,
    pub idx: usize,
    pub new_value: u8,
    pub prev_value: u8,
}
