extern crate termion;

use termion::{async_stdin, clear, cursor, style};
use termion::raw::IntoRawMode;
//use termion::event::Key;

use std::io::{stdout, Read, Write};
use std::time::{Instant, Duration};
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
            x_pos: 6,
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
    fn get_placement(&self) -> [[bool; 16]; 24] {
        let mut board: [[bool; 16]; 24] = [[false; 16]; 24];
        for i in 0..4 {
            for j in 0..4{
                board[i + self.y_pos][j + self.x_pos] = self.arr[self.shape][self.rotation][i][j];
            }
        }
        board
    }
}
//game struct
struct Game<R, W> {
    last_board: [[bool; 16]; 24],
    next_board: [[bool; 16]; 24],
    piece: Piece,
    piece_board: [[bool; 16]; 24],
    border_board: [[bool; 16]; 24],
    speed: u64,
    score: usize,
    running: bool,
    stdin: R,
    stdout: W,
}

fn init() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();

    let mut game = Game {
        last_board: [[false; 16]; 24],
        next_board: [[false; 16]; 24],
        //piece: Game::new_piece(),
        piece: Piece::new(rand::thread_rng().gen_range(0..7)),
        piece_board: [[false; 16]; 24],
        border_board: [
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true],
            [true, true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true, true, true, true],
            [true, true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true, true, true, true],
            [true, true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true, true, true, true]
        ],
        speed: 5,
        score: 0,
        running: true,
        stdin: stdin,
        stdout: stdout,
    };
    game.start();
}

impl<R: Read, W: Write> Game<R, W> {
    //game loop
    fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.reset();
        let mut before = Instant::now();

        loop {
            let interval = 1000 / self.speed;
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_millis()) as u64;

            if dt < interval {
                sleep(Duration::from_millis(interval - dt));
                continue;
            }

            before = now;

            if !self.update() {
                return;
            }
            
            self.shift_down();
            self.check_rows();
            self.last_board = self.next_board;

            self.render();

            write!(self.stdout, "{}", style::Reset).unwrap();
            self.stdout.flush().unwrap();
        }
    }
    
    fn reset(&mut self) {
        write!(self.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), style::Reset).unwrap();
        write!(self.stdout, "\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <! . . . . . . . . . .!>\r
     <!====================!>\r
       \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\r
\r
          SCORE: 0\n\r").unwrap();
        //TODO: add stdout flush
        self.score = 0;
        self.last_board = [[false; 16]; 24];
        self.next_board = [[false; 16]; 24];
        self.running = true;
        self.piece = self.new_piece();
        self.piece_board = self.piece.get_placement();
    }
    //make a new piece
    fn new_piece(&self) -> Piece {
        Piece::new(rand::thread_rng().gen_range(0..7))
    }
    //return true if there is a collision
    //NOTE: using last_board
    fn collides(&self) -> bool {
        for y in 0..24 {
            for x in 0..16 {
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
        for y in (1..21).rev() {
            //check if row is full
            full = true;
            x = 3;
            while full && x < 13 {
                full = self.last_board[y][x];
                x += 1;
            }
            //remove the row if it is full and move rows above down
            if full {
                self.score += 1; //TODO: add level score multiplier
                rows_removed += 1;
            }
            if y >= rows_removed {
                self.next_board[y] = self.last_board[y - rows_removed];
            } else {
                self.next_board[y] = [false; 16];
            }
        }
    }
    //update game struct
    //returns false if game is over or true if it is not
    fn update(&mut self) -> bool {
        //get key input
        let mut key_bytes = [0];
        self.stdin.read(&mut key_bytes).unwrap();
        match key_bytes[0] {
            b'h' | b'a'               => self.move_piece(Dir::Left),
            b'j' | b's'               => self.move_piece(Dir::Down),
            b'k' | b'w' | b'e' | b'v' => self.move_piece(Dir::Rcw),
            b'l' | b'd'               => self.move_piece(Dir::Right),
            b'i' | b'q' | b'c'        => self.move_piece(Dir::Rccw),
            b't'                      => return false,
            _ => {},
        };
            
        !self.collides()
    }
    //print board to console
    fn render(&mut self) {
        //print new board
        write!(self.stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
        for y in (1..21).rev() {
            for x in 3..13 {
                let block: &str = if self.next_board[y][x] || self.piece_board[y][x] {"[]"} else {" ."};
                let goto_y: u16 = (y + 1) as u16;
                let goto_x: u16 = ((x - 3) * 2 + 8) as u16;
                match x {
                    3 => write!(self.stdout, "{}{}", termion::cursor::Goto(goto_x, goto_y), block).unwrap(),
                    4..=12 => write!(self.stdout, "{}", block).unwrap(),
                    _ => {},
                };
            }
        }
        /*
        if self.running {
            println!("     <!====================!>");
        } else {
            println!("     <!=====GAME==OVER=====!>");
        }
        println!("       \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
        println!(" ");
        println!("            SCORE: {}", self.score);
        */
        write!(self.stdout, "{}{}", termion::cursor::Goto(18, 25), self.score).unwrap();
        self.stdout.flush().unwrap();
    }
    //move piece
    fn move_piece(&mut self, dir: Dir) {
        match dir {
            Dir::Left => if self.piece.x_pos > 1 {self.piece.x_pos -= 1} else {return},
            Dir::Right => if self.piece.x_pos < 15 {self.piece.x_pos += 1} else {return},
            Dir::Down => if self.piece.y_pos < 23 {self.piece.y_pos += 1} else {return},
            Dir::Rcw => if (self.piece.rotation < 3) {self.piece.rotation += 1} else {self.piece.rotation = 0},
            Dir::Rccw => if (self.piece.rotation > 0) {self.piece.rotation -= 1} else {self.piece.rotation = 3},
        }
        self.piece_board = self.piece.get_placement();
        if self.collides() {
            match dir {
                Dir::Left => self.piece.x_pos += 1,
                Dir::Right => self.piece.x_pos -= 1,
                Dir::Down => self.piece.y_pos -= 1,
                Dir::Rcw => if (self.piece.rotation > 0) {self.piece.rotation -= 1} else {self.piece.rotation = 3},
                Dir::Rccw => if (self.piece.rotation < 3) {self.piece.rotation += 1} else {self.piece.rotation = 0},
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
            self.piece = Piece::new(rand::thread_rng().gen_range(0..7));
            self.piece_board = self.piece.get_placement();
        }
    }
}
//main loop
fn main() {
    init();
}

