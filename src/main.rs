use std::time::Duration;
use std::thread::sleep;
use rand::Rng;

//movement directions
enum Dir {
    Left,
    Right,
    Down,
    Rcw,
    Rccw,
}
//piece struct
struct Piece {
    x_pos: usize,
    y_pos: usize,
    shape: usize,
    rotation: usize,
    arr: [[[[bool; 4]; 4]; 4]; 7],
}
impl Piece {
    fn new(s: usize) -> Piece {
        Piece {
            x_pos: 5,
            y_pos: 0,
            rotation: 0,
            shape: s,
            arr: [
                [
                    [
                        [false, false, false, false],
                        [true,  true,  true,  true],
                        [false, false, false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, true,  false],
                        [false, false, true,  false],
                        [false, false, true,  false],
                        [false, false, true,  false]
                    ],
                    [
                        [false, false, false, false],
                        [true,  true,  true,  true],
                        [false, false, false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, true,  false],
                        [false, false, true,  false],
                        [false, false, true,  false],
                        [false, false, true,  false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [true,  true,  true,  false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, true,  false, false],
                        [true,  true,  false, false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, true,  false, false],
                        [true,  true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, true,  false, false],
                        [false, true,  true,  false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [true,  true,  true,  false],
                        [true,  false, false, false],
                        [false, false, false, false]
                    ],
                    [
                        [true,  true,  false, false],
                        [false, true,  false, false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, false, true,  false],
                        [true,  true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, true,  false, false],
                        [false, true,  false, false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [true,  true,  true,  false],
                        [false, false, true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, true,  false, false],
                        [false, true,  false, false],
                        [true,  true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [true,  false, false, false],
                        [true,  true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, true,  true,  false],
                        [false, true,  false, false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [true,  true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [true,  false, false, false],
                        [true,  true,  false, false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [true,  true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [true,  false, false, false],
                        [true,  true,  false, false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [true,  true,  false, false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, true,  false],
                        [false, true,  true,  false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [true,  true,  false, false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, true,  false],
                        [false, true,  true,  false],
                        [false, true,  false, false],
                        [false, false, false, false]
                    ]
                ],
                [
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ],
                    [
                        [false, false, false, false],
                        [false, true,  true,  false],
                        [false, true,  true,  false],
                        [false, false, false, false]
                    ]
                ]
            ],
        }
    }
    //return piece on empty board
    fn get_placement(&self) -> [[bool; 14]; 23] {
        let mut board: [[bool; 14]; 23] = [[false; 14]; 23];
        for i in 0..4 {
            for j in 0..4{
                board[i + self.y_pos][j + self.x_pos] = self.arr[self.shape][self.rotation][i][j];
            }
        }
        board
    }
    /*
    //rotate cw
    fn rotate_cw(&mut self) {
        //TODO: add check for collision
        //self.rotation = if self.rotation == 3 { 0 } else { self.rotation + 1 };
        self.rotation = match self.rotation {
            3 => 0,
            _ => self.rotation + 1,
        }
    }
    //rotate ccw
    fn rotate_ccw(&mut self) {
        //TODO: add check for collision
        self.rotation = match self.rotation {
            0 => 3,
            _ => self.rotation - 1,
        }
    }
    //move left
    fn move_left(&mut self) {
        //TODO: add check for collision
        self.pos = match self.pos % 10 {
            0 => self.pos,
            _ => self.pos - 1,
        }
    }
    //move right
    fn move_right(&mut self) {
        //TODO: add check for collision
        self.pos = match self.pos % 10 {
            9 => self.pos,
            _ => self.pos + 1,
        }
    }
    //move down
    fn move_down(&mut self) {
        //TODO: add check for collision
        self.pos = match self.pos {
            190 ..= 199 => self.pos,//call end in this case?
            _ => self.pos + 10,
        }
    }
    */
    //drop
        //while no collision move down
    //end
        //release struct and make a new one
}
//game struct
struct Game {
    last_board: [[bool; 14]; 23],
    next_board: [[bool; 14]; 23],
    piece: Piece,
    piece_board: [[bool; 14]; 23],
    border_board: [[bool; 14]; 23],
    frame_time: Duration,//should this be replaced with level? and then a match for time in loop?
    score: usize,
}
impl Game {
    fn new(millis: u64) -> Game {
        let duration: Duration = std::time::Duration::from_millis(millis);
        let first_piece = Game::new_piece();
        let first_piece_board = first_piece.get_placement();
        let mut border: [[bool; 14]; 23] = [[false; 14]; 23];
        for i in 0..21 { border[i] = [true, true, false, false, false, false, false, false, false, false, false, false, true, true] };
        for i in 21..23 { border[i] = [true; 14] };
        Game {
            last_board: [[false; 14]; 23],
            next_board: [[false; 14]; 23],
            piece: first_piece,
            piece_board: first_piece_board,
            border_board: border,
            frame_time: duration,
            score: 0,
        }
    }
    //make a new piece
    fn new_piece() -> Piece {
        Piece::new(rand::thread_rng().gen_range(0..7))
    }
    //return true if there is a collision
    //NOTE: using last_board
    fn collides(&self) -> bool {
        for y in 0..23 {
            for x in 0..14 {
                if self.piece_board[y][x] && (self.last_board[y][x] || self.border_board[y][x]) {
                    return true;
                }
            }
        }
        false
    }
    //check for a full row (checks last_board and updates next_board)
    fn check_rows(&mut self) {
        let mut full: bool;
        let mut rows_removed: usize = 0;
        let mut x: usize;
        for y in (2..23).rev() {
            //check if row is full
            full = true;
            x = 2;
            while full && x < 12 {
                full = self.last_board[y][x];
                x += 1;
            }
            //remove the row if it is full and move rows above down
            if full {
                self.score += 1; //TODO: add level score multiplier
                rows_removed += 1;
            }
            if y - rows_removed > 0 {
                self.next_board[y] = self.last_board[y - rows_removed];
            } else {
                self.next_board[y] = [false; 14];
            }
        }
    }
    //update game struct
    fn update(&mut self) {
        //update to next frame
        self.shift_down();
        self.last_board = self.next_board;
    }
    //print board to console
    //note the piece needs to be overlaid on the board instead of being included
    fn render(&self) {
        //print new board
        //TODO: clear console and set cursor to top left
        println!(" ");
        for y in (1..22).rev() {
            for x in 2..12 {
                let block: &str = if self.next_board[y][x] || self.piece_board[y][x] {"[]"} else {" ."};
                match x {
                    2 => print!("     <!{}", block),
                    3..=10 => print!("{}", block),
                    11 => print!("{}!>", block),
                    _ => {},
                };
            }
        }
        println!("     <!====================!>");
        println!("       \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
    }
    //move piece
    fn move_piece(&mut self, dir: Dir) {
        match dir {
            Dir::Left => if self.piece.x_pos > 0 {self.piece.x_pos -= 1} else {return},
            Dir::Right => if self.piece.x_pos < 14 {self.piece.x_pos += 1} else {return},
            Dir::Down => if self.piece.y_pos < 23 {self.piece.y_pos += 1} else {return},
            Dir::Rcw => self.piece.rotation += 1,
            Dir::Rccw => self.piece.rotation -= 1,
        }
        self.piece_board = self.piece.get_placement();
        if self.collides() {
            match dir {
                Dir::Left => self.piece.x_pos += 1,
                Dir::Right => self.piece.x_pos -= 1,
                Dir::Down => self.piece.y_pos -= 1,
                Dir::Rcw => self.piece.rotation -= 1,
                Dir::Rccw => self.piece.rotation += 1,
            }
        }
        self.piece_board = self.piece.get_placement();
    }
    //shift piece down (for game tick movement, not player movement)
    //NOTE: updates last_board
    fn shift_down(&mut self) {
        let mut end: bool = false;
        if self.piece.y_pos < 23 {
            self.piece.y_pos += 1;
            self.piece_board = self.piece.get_placement();
            if self.collides() {
                end = true;
                self.piece.y_pos -= 1;
            }
        } else {
            end = true
        }
        //if piece is as far down as it can go put it in board and make new piece
        if end {
            self.piece_board = self.piece.get_placement();
            for row in self.last_board.iter_mut().zip(self.piece_board.iter()) {
                for cell in row.0.iter_mut().zip(row.1.iter()) {
                    *cell.0 = *cell.0 || *cell.1;
                }
            }
            //NOTE: is there supposed to be a 1 tick delay before the new piece shows up?
            //if there is there needs to be a new state for that
            self.piece = Game::new_piece();
            self.piece_board = self.piece.get_placement();
        }
    }
}
//main loop
fn main() {
    let mut game = Game::new(100);
    let mut running = true;
    while running {
        game.update();
        game.render();
        sleep(game.frame_time);
    };
}

