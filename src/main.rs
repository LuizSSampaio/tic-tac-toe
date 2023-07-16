fn main() {
    let mut running = true;
    let mut is_x_actual_player = true;
    let mut board = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
    while running {
        write_screen(board);
        ask_play(is_x_actual_player, &mut board);
        running = !has_won(board);
        if running {
            is_x_actual_player = !is_x_actual_player;
        } else {
            write_screen(board);
            if is_x_actual_player {
                println!("The player X won the game");
            } else {
                println!("The player O won the game");
            }
        }
    }
}

fn has_won(board: [[&str; 3]; 3]) -> bool {
    let mut won = false;

    // Row win validation
    for row in board {
        let mut last_mark: &str = "";
        let mut possibility_of_win = true;
        for mark in row {
            if last_mark != "" && possibility_of_win {
                if mark != last_mark {
                    possibility_of_win = false;
                }
            } else if last_mark == "" {
                last_mark = mark;
            }
        }
        if possibility_of_win {
            won = true;
        }
    }

    // Column win validation
    let mut column_id = 0;
    while column_id < 3 {
        let mut last_mark: &str = "";
        let mut possibility_of_win = true;
        for row in board {
            if last_mark != "" && possibility_of_win {
                if row[column_id] != last_mark {
                    possibility_of_win = false;
                }
            } else if last_mark == "" {
                last_mark = row[column_id];
            }
        }
        if possibility_of_win {
            won = true;
        }
        column_id += 1;
    }

    // Diagonal win validation
    let mut diagonal_id = 0;
    while diagonal_id < 2 {
        let mut possibility_of_win = true;

        if board[0][0] != board[1][1] && board[0][2] != board[1][1] {
            possibility_of_win = false;
        } else if board[0][0] != board[2][2] && board[0][2] != board[2][0] {
            possibility_of_win = false
        }

        if possibility_of_win {
            won = true;
        }
        diagonal_id += 1
    }

    won
}

fn ask_play(is_x_actual_player: bool, board: &mut [[&str; 3]; 3]) {
    loop {
        let actual_player: &str;
        if is_x_actual_player {
            actual_player = "X";
        } else {
            actual_player = "O";
        }
        println!("The actual player is: {}", actual_player);
        println!("Enter the position to mark: ");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("This field doesn't exist.");
                continue;
            }

            if !mark_position(board, actual_player, number) {
                continue;
            }
            break;
        }
    }
}

fn mark_position<'a>(board: &mut [[&'a str; 3]; 3], mark: &'a str, position_id: usize) -> bool {
    if position_id <= 3 {
        if board[0][position_id - 1] != "X" && board[0][position_id - 1] != "O" {
            board[0][position_id - 1] = mark;
            return true;
        }
    } else if position_id <= 6 {
        if board[1][position_id - 4] != "X" && board[1][position_id - 4] != "O" {
            board[1][position_id - 4] = mark;
            return true;
        }
    } else {
        if board[2][position_id - 7] != "X" && board[2][position_id - 7] != "O" {
            board[2][position_id - 7] = mark;
            return true;
        }
    }
    false
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
