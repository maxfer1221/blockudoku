use crate::*;

use nalgebra as na;
use na::{U2, SMatrix};

pub type Matrix9x9 = SMatrix<u8, 9, 9>;

#[derive(Clone, Debug)]
pub struct Board {
    cells: Matrix9x9,
}

impl Board {
    pub fn new() -> Self {
        Self { cells: SMatrix::zeros() }
    }

    pub fn place(&mut self, piece_as_matrix: &Matrix9x9) {
        self.cells += piece_as_matrix;
        self.check_grid();
    }

    fn check_grid(&mut self) {
        todo!()
    }

    pub fn is_zero(&self) -> bool {
       let matrix = self.cells;
        
       // return if any element is nonzero
       !matrix
           .iter()
           .any(|i| *i != 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn new_boad_is_empty() { 
        let board: Board = Board::new();
        assert!(board.is_zero());
    }
}
