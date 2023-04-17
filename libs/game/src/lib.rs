pub use self::{board::*};

mod board;

pub struct Game {
    board: Board,
    score: u32,
//    pieces: [Piece; 3],
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        let score = 0u32;
        // let pieces = (0..3)
        //    .map(|_| Piece::random())
        //    .collect();

        Self { board, score } //, pieces }
    }
}
