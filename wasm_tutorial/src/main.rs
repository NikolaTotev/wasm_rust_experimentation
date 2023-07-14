fn main() {
    println!("For 0,0: ");
    live_neighbor_count(0,0);
    println!();

    println!("For 2,2: ");
    live_neighbor_count(2,2);
}

fn get_index(row: u32, column: u32) -> usize {
    (row *24 + column) as usize
}

fn live_neighbor_count(row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [5 - 1, 0, 1].iter().cloned() {
        for delta_col in [5 - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (row + delta_row) % 5;
            let neighbor_col = (column + delta_col) % 5;
            let idx = get_index(neighbor_row, neighbor_col);
            println!("Row: {neighbor_row} | Col: {neighbor_col} | Index {idx} ")
        }
    }
    count
}
