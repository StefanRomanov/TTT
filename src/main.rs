use std::io::{stdin};
use log::error;

fn main() {
    let mut board: [[u8; 3]; 3] = [[0;3]; 3];
    println!("Lets play Tic Tac Toe !");
    println!("Each player input should be represented as integers on separate line. Enter row and column in that order");
    print_board(board);

    loop {
        for player in 1..3  {
            if play(&mut board, player) {
                break
            }
        }

    }
}

fn check_board(board: [[u8; 3]; 3], player: u8, row: usize, col: usize) -> bool {
    return check_vertical(board, col, player) ||
        check_horizontal(board, row, player) ||
        check_diagonal(board, row, col, player);
}

fn mark_board(board: &mut[[u8; 3]; 3], player: u8, x: usize, y: usize) -> bool {
    if board[x][y] == 0  {
        board[x][y] = player;
        return true
    }

    println!("Invalid input values");
    false
}

fn check_vertical(board: [[u8; 3]; 3], col: usize, player: u8) -> bool {
    for row_array in board.iter() {
        if row_array[col] != player {
            return false;
        }
    }
    println!("vertical");
    return true
}

fn check_horizontal(board: [[u8; 3]; 3], row: usize, player: u8) -> bool {
    for elem in board[row].iter() {
        if (*elem != player) {
            return false;
        }
    }

    println!("horizontal");
    return true;
}

fn check_diagonal(board: [[u8; 3]; 3], row: usize, col: usize, player: u8) -> bool {
    if (col == 1 && row != 1 || row == 1 && col != 1) {
        return false;
    }

    if (row == col) {
        return board[0][0] == player && board[1][1] == player && board[2][2] == player;
    }

    return return board[0][2] == player && board[1][1] == player && board[2][0] == player;
}

fn print_board(board: [[u8; 3]; 3]) {
    for row in board {
        println!("{r:?}", r=row)
    }
}

fn handle_input() -> usize {
    let mut input = String::new();
    let mut val:  usize = 5;

    while val > 2 {
        stdin().read_line(&mut input).expect("Did not enter a correct number");
        val = input.trim().parse::<usize>().unwrap_or_else(|err| {
            println!("Invalid input");
            5
        });
        input.clear();
    }

    return val;
}

fn play(board: &mut[[u8; 3]; 3], player: u8) -> bool {
    let mut col: usize;
    let mut row: usize;

    println!("Player {} engage !", player);
    col = handle_input();
    row = handle_input();

    while !mark_board(board, player, col, row) {
        col = handle_input();
        row = handle_input();
    }
    let current_result = check_board(*board, player, col, row);
    print_board(*board);

    if current_result {
        println!("Player {} won!", player);
        return true
    }

    false
}
