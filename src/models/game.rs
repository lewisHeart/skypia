use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicTacToeCell {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicTacToe {
    pub board: [TicTacToeCell; 9],
    pub turn: TicTacToeCell,
    pub winner: Option<TicTacToeCell>,
    pub is_draw: bool,
    pub active: bool,
    pub accepted: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TicTacToeCell::Empty; 9],
            turn: TicTacToeCell::X,
            winner: None,
            is_draw: false,
            active: true,
            accepted: false,
        }
    }
}
