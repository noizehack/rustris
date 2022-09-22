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
    frame_time: Duration,//should this be replaced with level? and then a match for time in loop?
    score: usize,
}
impl Game {
    fn new(millis: u64) -> Game {
        let duration: Duration = std::time::Duration::from_millis(millis);
        let mut rng = rand::thread_rng();
        let first_piece = Piece::new(rng.gen_range(0..7));
        Game {
            last_board: [false; 210],
            next_board: [false; 210],
            piece: first_piece,
            frame_time: duration,
            score: 0,
        }
    }
    //return true if there is a collision
    fn collides(&self) -> bool {
        let piece_arr: [bool; 210] = self.piece.get_placement();
        for i in self.last_board.iter().zip(piece_arr.iter()) {
            if !*i.0 && !*i.1 {
                return true;
            }
        }
        false
    }
    //check for a full row (checks last_board and updates next_board)
    //note last_board can't include the current piece
    fn check_rows(&mut self) {
        let mut full: bool;
        let mut j: usize;
        let mut full_rows: usize = 0;
        for i in 0..20 {
            full = true;
            j = 0;
            while full && j < 10 {
                if self.last_board[i * 10 + j] == ' ' {
                    full = false;
                }
                j += 1;
            }
            if full {
                full_rows += 1;
                self.score += 1;
            }
            for x in 0..10 {
                let row_start: usize = (i + full_rows) * 10 ;
                //TODO FINISH!!!
            }
        }
        //remove rows in full_rows vec
        for i in  {
            
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
        for i in 0..199 {
            match (i + 1) % 10 {
                0 => println!("{}", self.next_board[i]),
                _ => print!("{}", self.next_board[i]),
            };
        }
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

