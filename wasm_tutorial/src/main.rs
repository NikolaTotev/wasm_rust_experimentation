fn main() {
    for delta_row in [25 - 1, 0, 1].iter().cloned() {
        for delta_col in [20 - 1, 0, 1].iter().cloned(){
            //println!("{delta_col} {delta_row}");
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (0 + delta_row) % 25;
            let neighbor_col = (2 + delta_col) % 20;

            println!("Neighbors: {neighbor_row} {neighbor_col}");
        }
        
    }}
