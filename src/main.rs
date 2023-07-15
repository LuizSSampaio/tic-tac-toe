fn main() {
    let mut running = true;
    let mut board = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
    while running {
        mark_position(&mut board, "X", 7);
        write_screen(board);
        running = false;
    }
}

fn mark_position<'a>(board: &mut [[&'a str; 3]; 3], mark: &'a str, position_id: usize) {
    if position_id <= 3 {
        board[0][position_id - 1] = mark;
    } else if position_id <= 6 {
        board[1][position_id - 4] = mark;
    } else {
        board[2][position_id - 7] = mark;
    }
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
