use crate::state::AppState;
use crate::models::{Message, TicTacToe, TicTacToeCell};
use dioxus::prelude::*;

impl AppState {
    pub fn start_game(&mut self, contact_id: String) {
        let msg_id = self.message_counter();
        let u_name = self.user_name();

        let mut games = self.game_states.write();
        games.insert(contact_id.clone(), TicTacToe::new());

        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_msg = Message {
            id: msg_id.to_string(),
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
            sender_name: u_name,
            text: "Iniciou uma partida de Jogo da Velha.".to_string(),
            timestamp: now,
            is_nudge: false,
            font_color: "#2e6930".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: true,
        };
        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_msg);
    }

    pub fn make_game_move(&mut self, contact_id: String, cell_idx: usize) {
        let mut games = self.game_states.write();
        if let Some(game) = games.get_mut(&contact_id) {
            if !game.active
                || game.board[cell_idx] != TicTacToeCell::Empty
                || game.turn != TicTacToeCell::X
            {
                return;
            }

            game.board[cell_idx] = TicTacToeCell::X;

            if check_game_over(game) {
                return;
            }

            game.turn = TicTacToeCell::O;
        }
    }
}

fn check_game_over(game: &mut TicTacToe) -> bool {
    let win_patterns = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for p in &win_patterns {
        if game.board[p[0]] != TicTacToeCell::Empty
            && game.board[p[0]] == game.board[p[1]]
            && game.board[p[0]] == game.board[p[2]]
        {
            game.winner = Some(game.board[p[0]]);
            game.active = false;
            return true;
        }
    }

    if game.board.iter().all(|c| *c != TicTacToeCell::Empty) {
        game.is_draw = true;
        game.active = false;
        return true;
    }

    false
}
