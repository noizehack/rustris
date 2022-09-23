use std::time::Duration;
use std::thread::sleep;
use rand::Rng;

//piece struct
struct Piece {
    pos: usize,
    shape: usize,
    rotation: usize,
    arr: [[[bool; 16]; 4]; 7],
}
impl Piece {
    fn new(s: usize) -> Piece {
        Piece {
            pos: 3,
            rotation: 0,
            shape: s,
            arr: [
                [
                    [
                        false, false, false, false,
                        true,  true,  true,  true,
                        false, false, false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, true,  false,
                        false, false, true,  false,
                        false, false, true,  false,
                        false, false, true,  false
                    ],
                    [
                        false, false, false, false,
                        true,  true,  true,  true,
                        false, false, false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, true,  false,
                        false, false, true,  false,
                        false, false, true,  false,
                        false, false, true,  false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        true,  true,  true,  false,
                        false, true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, true,  false, false,
                        true,  true,  false, false,
                        false, true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, true,  false, false,
                        true,  true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, true,  false, false,
                        false, true,  true,  false,
                        false, true,  false, false,
                        false, false, false, false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        true,  true,  true,  false,
                        true,  false, false, false,
                        false, false, false, false
                    ],
                    [
                        true,  true,  false, false,
                        false, true,  false, false,
                        false, true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, false, true,  false,
                        true,  true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, true,  false, false,
                        false, true,  false, false,
                        false, true,  true,  false,
                        false, false, false, false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        true,  true,  true,  false,
                        false, false, true,  false,
                        false, false, false, false
                    ],
                    [
                        false, true,  false, false,
                        false, true,  false, false,
                        true,  true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        true,  false, false, false,
                        true,  true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, true,  true,  false,
                        false, true,  false, false,
                        false, true,  false, false,
                        false, false, false, false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        true,  true,  false, false,
                        false, false, false, false
                    ],
                    [
                        true,  false, false, false,
                        true,  true,  false, false,
                        false, true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        true,  true,  false, false,
                        false, false, false, false
                    ],
                    [
                        true,  false, false, false,
                        true,  true,  false, false,
                        false, true,  false, false,
                        false, false, false, false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        true,  true,  false, false,
                        false, true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, false, true,  false,
                        false, true,  true,  false,
                        false, true,  false, false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        true,  true,  false, false,
                        false, true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, false, true,  false,
                        false, true,  true,  false,
                        false, true,  false, false,
                        false, false, false, false
                    ]
                ],
                [
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        false, true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        false, true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        false, true,  true,  false,
                        false, false, false, false
                    ],
                    [
                        false, false, false, false,
                        false, true,  true,  false,
                        false, true,  true,  false,
                        false, false, false, false
                    ]
                ]
            ],
        }
    }
    //return piece on empty board
    fn get_placement(&self) -> [bool; 210] {
        let mut board: [bool; 210] = [false; 210];
        for i in 0..16 {
            board[i + self.pos] = self.arr[self.shape][self.rotation][i];
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
    last_board: [bool; 210],
    next_board: [bool; 210],
    piece: Piece,
    piece_board: [bool; 210],
    frame_time: Duration,//should this be replaced with level? and then a match for time in loop?
    score: usize,
}
impl Game {
    fn new(millis: u64) -> Game {
        let duration: Duration = std::time::Duration::from_millis(millis);
        let mut rng = rand::thread_rng();
        let first_piece = Piece::new(rng.gen_range(0..7));
        let first_piece_board = first_piece.get_placement();
        Game {
            last_board: [false; 210],
            next_board: [false; 210],
            piece: first_piece,
            piece_board: first_piece_board,
            frame_time: duration,
            score: 0,
        }
    }
    //return true if there is a collision
    fn collides(&self) -> bool {
        for i in self.last_board.iter().zip(self.piece_board.iter()) {
            if *i.0 && *i.1 {
                return true;
            }
        }
        false
    }
    //check for a full row (checks last_board and updates next_board)
    fn check_rows(&mut self) {
        let mut full: bool;
        let mut j: usize;
        for i in 0..21 {
            full = true;
            j = 0;
            while full && j < 10 {
                full = self.last_board[i * 10 + j];
                j += 1;
            }
            if full {
                self.score += 1; //TODO: add level score multiplier
                for x in 0..10 {
                    self.next_board[i * 10 + x] = self.last_board[(i + 1) * 10 + x];
                }
            } else {
                for x in 0..10 {
                    self.next_board[i * 10 + x] = self.last_board[i * 10 + x];
                }
            }
        }
    }
    //update game struct
    fn update(&mut self) {
        //update to next frame
    }
    //print board to console
    //note the piece needs to be overlaid on the board instead of being included
    fn render(&self) {
        //print new board
        //TODO: print piece on top of board
        //TODO: clear console and set cursor to top left
        println!(" ");
        for i in (0..21).rev() {
            for j in 0..10 {
                let block: &str = if self.next_board[i * 10 + j] {"[]"} else {" ."};
                match j {
                    0 => print!("     <!{}", block),
                    1..=8 => print!("{}", block),
                    9 => print!("{}!>", block),
                    _ => {},
                };
            }
        }
        println!("     <!====================!>");
        println!("       \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
    }
    //move piece left
    fn move_left(&mut self) {

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

