use std::fmt;

pub struct Cell {
    pub row: u8,
    pub column: u8,
    pub value: u8,
    pub previous_value: Option<u8>,
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
