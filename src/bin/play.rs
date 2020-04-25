use sudoku::types::cell;

fn main() {
    let cell = cell::Cell {
        row: 1,
        column: 2,
        ..Default::default()
    };
    let mut peers = cell.peers();
    peers.sort();
    println!("peers: {:?}, len: {}", peers, peers.len());
}
