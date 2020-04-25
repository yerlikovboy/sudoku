// A group is made up of a  row, column, and the corresponding block.
// A classic 9x9 Sudoku has 27 groups. Each digit appears once in each group.
struct Peers {
    _cells: HashMap<u8, Vec<u8>>,
}

impl Group {
    #[allow(dead_code)]
    pub fn get(row: u8, column: u8, puzzle: &Puzzle) {}
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_id() {
        assert_eq!(Group::group_id(0, 0), 1);
        assert_eq!(Group::group_id(0, 1), 1);
        assert_eq!(Group::group_id(0, 2), 1);

        assert_eq!(Group::group_id(0, 3), 2);
        assert_eq!(Group::group_id(0, 4), 2);
        assert_eq!(Group::group_id(0, 5), 2);

        assert_eq!(Group::group_id(0, 6), 3);
        assert_eq!(Group::group_id(0, 7), 3);
        assert_eq!(Group::group_id(0, 8), 3);

        assert_eq!(Group::group_id(1, 0), 1);
        assert_eq!(Group::group_id(1, 1), 1);
        assert_eq!(Group::group_id(1, 2), 1);

        assert_eq!(Group::group_id(1, 3), 2);
        assert_eq!(Group::group_id(1, 4), 2);
        assert_eq!(Group::group_id(1, 5), 2);

        assert_eq!(Group::group_id(1, 6), 3);
        assert_eq!(Group::group_id(1, 7), 3);
        assert_eq!(Group::group_id(1, 8), 3);

        assert_eq!(Group::group_id(3, 4), 5);
        assert_eq!(Group::group_id(8, 8), 9);
    }
}
*/
