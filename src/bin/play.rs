use sudoku::types::cell;
use sudoku::types::puzzle;

pub fn print_console(p: &puzzle::Puzzle) {
    let _grid = p.grid_as_ref();
    println!("Puzzle");
    for i in 0..9 {
        if i % 3 == 0 {
            println!("-------------------------");
        }
        for j in 0..9 {
            if j % 3 == 0 {
                print!("| ");
            }
            let v = _grid[(i * 9) + j];
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

fn dump(c: &cell::Cell) {
    println!("cell: {}", c);
    let mut v: [u8; 81] = [0; 81];
    c.peers().iter().for_each(|x| v[*x as usize] = 1);
    let p = puzzle::Puzzle::new(&v[..]);
    print_console(&p);
}

fn main() {
    let cells: [cell::Cell; 12] = [
        cell::Cell {
            row: 1,
            column: 2,
            ..Default::default()
        },
        cell::Cell {
            row: 2,
            column: 3,
            ..Default::default()
        },
        cell::Cell {
            row: 0,
            column: 7,
            ..Default::default()
        },
        cell::Cell {
            row: 3,
            column: 3,
            ..Default::default()
        },
        cell::Cell {
            row: 4,
            column: 1,
            ..Default::default()
        },
        cell::Cell {
            row: 4,
            column: 4,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 2,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 5,
            ..Default::default()
        },
        cell::Cell {
            row: 5,
            column: 8,
            ..Default::default()
        },
        cell::Cell {
            row: 6,
            column: 6,
            ..Default::default()
        },
        cell::Cell {
            row: 7,
            column: 7,
            ..Default::default()
        },
        cell::Cell {
            row: 8,
            column: 8,
            ..Default::default()
        },
    ];

    for cell in &cells[..] {
        dump(&cell);
    }
}
