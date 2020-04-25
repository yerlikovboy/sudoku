use std::collections::HashSet;
use std::fmt;

#[derive(Default)]
pub struct Cell {
    pub row: u8,
    pub column: u8,
    pub value: Option<u8>,
    pub previous_value: Option<u8>,
}

impl Cell {
    pub fn new(row: u8, column: u8, value: Option<u8>) -> Result<Cell, String> {
        if row < 1 || row > 9 {
            return Err(String::from("row must be in the range 1..9"));
        }

        if column < 1 || column > 9 {
            return Err(String::from("column must be in the range 1..9"));
        }
        if value.unwrap_or(0) > 9 {
            return Err(String::from("new value must be between 0 and 9"));
        }

        Ok(Cell {
            row: row - 1,
            column: column - 1,
            value: value,
            previous_value: Option::None,
        })
    }

    pub fn to_grid_idx(&self) -> usize {
        (self.row as usize * 9) + self.column as usize
    }

    pub fn from_grid_idx(idx: usize) -> Cell {
        Cell {
            row: (idx as u8 / 9) * 9,
            column: idx as u8 % 9,
            ..Default::default()
        }
    }

    pub fn peers(&self) -> Vec<usize> {
        println!("peers of {}", self);
        let i: usize = self.to_grid_idx();
        let row_idx: usize = (i / 9) * 9;
        let row_end: usize = row_idx + 9;
        let rows: HashSet<usize> = (row_idx..row_end).filter(|x| *x != i).collect();
        let col_offset: usize = i % 9;
        let col: HashSet<usize> = (0..9)
            .map(|x| (x * 9) + col_offset)
            .filter(|x| *x != i)
            .collect();

        let s: HashSet<usize> = rows.union(&col).cloned().collect();

        println!(
            "i: {}, \nrow_range: {}..{}, \n\trows: {:?},\ncol_offset:{}, \n\tcol_idx:{:?}",
            i, row_idx, row_end, rows, col_offset, col
        );

        s.iter().cloned().collect()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //let previous_value = self.previous_value.unwrap_or(0);
        let previous_value = self
            .previous_value
            .map_or(String::from("none"), |x| x.to_string());

        let val = self.value.map_or(String::from("none"), |x| x.to_string());

        formatter.write_fmt(format_args!(
            "cell: [row: {}, column: {}, value: {}, previous_value: {}]",
            self.row, self.column, val, previous_value
        ))
    }
}
