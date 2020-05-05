use crate::game::cell::Cell;
use std::fmt;
use std::time::SystemTime;

type UpdateId = u64;

pub struct Request {
    id: UpdateId,
    idx: usize,
    value: u8,
}

fn gen_id() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Request {
    pub fn new(row: usize, column: usize, value: u8) -> Request {
        Request {
            id: gen_id(),
            idx: Cell::to_grid_idx(row, column),
            value,
        }
    }

    pub fn from_idx(idx: usize, value: u8) -> Request {
        Request {
            id: gen_id(),
            idx,
            value,
        }
    }

    pub fn id(&self) -> UpdateId {
        self.id
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

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "id: {}, cell: {}, value: {}",
            self.id,
            Cell::from_grid_idx(self.idx),
            self.value
        ))
    }
}

#[derive(Default)]
pub struct Response {
    pub id: UpdateId,
    pub req_id: UpdateId,
    pub idx: usize,
    pub new_value: u8,
    pub prev_value: u8,
}

impl Response {
    pub fn undo_request(&self) -> Request {
        Request::from_idx(self.idx, self.prev_value)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "[num: {}, cell: {}, value: {}, prev: {}]",
            self.id,
            Cell::from_grid_idx(self.idx),
            self.new_value,
            self.prev_value
        ))
    }
}
