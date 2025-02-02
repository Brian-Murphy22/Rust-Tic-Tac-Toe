use std::{fmt, io};

struct Player {
    name: String,
    piece: Piece,
}

/// The make_move function loops until the player makes a valid board spot.
/// When the player picks a valid board spot, it will parse the string into an int. 
/// When the string is parsed into an int, the int needs to be set to an appropriate value. 
/// These appropriate values are values in the board array. 
/// If the input value isn't in the board array, the user gets prompted for a different value.
impl Player {
    fn make_move(&self, board: &mut Vec<Piece>) {
        // loop until the player provides a valid board spot
        loop {
            let mut input = String::new();
            println!("{} Pick an empty space on the board: ", self.name);
            match io::stdin().read_line(&mut input) {
                // if things are okay we want to try and parse the string into an int
                Ok(_) => {
                    match input.trim().parse::<usize>() {
                        // if we can parse the string into an int then we want to set
                        // the value appropirately
                        Ok(cell) => {
                            println!("You chose cell {}", cell);
                            board[cell] = self.piece;
                        }
                        // If things go wrong prompt the user again
                        Err(_) => {
                            println!("Sorry, {} isn't a space on the board", input.trim());
                            self.make_move(board);
                        }
                    }
                }
                // if things go wrong propmt the user again
                Err(e) => {
                    println!("Error, {}", e);
                    self.make_move(board);
                }
            }
            break;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Piece {
    X,
    O,
    Empty,
}

/// The fmt function is for what symbols get displayed on the board.
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Piece::X => write!(f, "X"),
            Piece::O => write!(f, "O"),
            Piece::Empty => write!(f, " "),
        }
    }
}

/// The draw_board function draws the board array in the terminal.
/// The integers within the array are the valid values players can input.
fn draw_board(board: &Vec<Piece>) {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

/// The tie function determines whether or not there is a tie game.
fn tie(board: &Vec<Piece>) -> bool {
    for value in board.iter() {
        if let Piece::Empty = value {
            return false;
        }
    }
    true
}

/// The initialize_board function will display an empty board in the terminal. 
fn initialize_board() -> Vec<Piece> {
    let mut board: Vec<Piece> = vec![];
    for _ in 0..9 {
        board.push(Piece::Empty)
    }
    board
}
/// The three_in_a_row function will determine if three X's or O's occur in a row.
fn three_in_a_row(one: Piece, two: Piece, three: Piece) -> bool {
    if (one == two) & (one == three) & (one != Piece::Empty) {
        return true;
    }
    false
}

/// The win function determines if a player has won by using the three_in_a_row function to test all possible orientations in which a player can win.
// 0, 1, 2
// 3, 4, 5
// 6, 7, 8
fn win(board: &Vec<Piece>) -> bool {
    // check that there is a 3 in a row on the rows
    if three_in_a_row(board[0], board[1], board[2])
        | three_in_a_row(board[3], board[4], board[5])
        | three_in_a_row(board[6], board[7], board[8])
        // check that there is a 3 in a row on the columns
        | three_in_a_row(board[0], board[3], board[6])
        | three_in_a_row(board[1], board[4], board[7])
        | three_in_a_row(board[2], board[5], board[8])
        //check that there is a 3 in a row on the diagonals
        | three_in_a_row(board[0], board[4], board[8])
        | three_in_a_row(board[6], board[4], board[2])
    {
        return true;
    }
    false
}

/// The main function first initializes the board.
/// It then assigns each player an X or an O.
/// It then draws the board in the terminal.
/// It loops players making moves and redraws the board after each move.
/// After each redraw, it checks if a player has won or if there is a tie and prints a message depending on which. 
fn main() {
    let mut board = initialize_board();
    let player1 = Player {
        piece: Piece::X,
        name: String::from("Player1"),
    };
    let player2 = Player {
        piece: Piece::O,
        name: String::from("Player2"),
    };

    let mut p1_turn = true;

    draw_board(&board);
    loop {
        if p1_turn {
            player1.make_move(&mut board);
        } else {
            player2.make_move(&mut board);
        }
        draw_board(&board);

        if win(&board) {
            println!("Congrats!!");
            break;
        }

        if tie(&board) {
            print!("Tie Game");
            break;
        }

        // at the end of the turn switch the player
        p1_turn = !p1_turn
    }
}
