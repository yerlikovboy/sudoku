use std::fmt;

#[derive(Default)]
pub struct Cell {
    pub row: u8,
    pub column: u8,
    pub value: Option<u8>,
    pub previous_value: Option<u8>,
}

impl Cell {
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
        let _self_idx = self.to_grid_idx();
        let mut _mapper: [bool; 81] = [false; 81];

        //rows
        let row_idx: usize = (_self_idx / 9) * 9;
        let row_end: usize = row_idx + 9;
        (row_idx..row_end)
            .filter(|x| *x != _self_idx)
            .for_each(|x| _mapper[x] = true);

        // cols
        let col_offset: usize = _self_idx % 9;
        (0..9)
            .map(|x| (x as usize * 9) + col_offset)
            .filter(|x| *x != _self_idx)
            .for_each(|x| _mapper[x] = true);

        // block
        let block_x: usize = (self.row as usize / 3) * 3;
        let block_y: usize = (self.column as usize / 3) * 3;
        for i in 0..3 {
            let row_offset = (block_x + i) * 9;
            for j in 0..3 {
                let col_idx = block_y + j;
                let idx = row_offset + col_idx;
                if _self_idx != idx {
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
