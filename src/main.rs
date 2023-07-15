fn main() {
    let board = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
    write_screen(board);
}

fn write_screen(board: [[&str; 3]; 3]) {
    println!("{}", "-".repeat(13));
    for row in board {
        let mut column_number = 1;
        let row_size = row.len();
        for column in row {
            if row_size != column_number {
                print!("| {} ", column);
            } else {
                println!("| {} |", column);
            }
            column_number += 1;
        }
        println!("{}", "-".repeat(13));
    }
}
