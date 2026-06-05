use crate::models::{Message, TicTacToe, TicTacToeCell};
use crate::state::AppState;
use dioxus::prelude::*;

impl AppState {
    pub fn start_game(&mut self, contact_id: String) {
        let msg_id = self.message_counter();
        let u_name = self.user_name();

        {
            let mut games = self.game_states.write();
            let mut new_game = TicTacToe::new();
            new_game.accepted = false; // Começa pendente
            games.insert(contact_id.clone(), new_game);
        }

        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_msg = Message {
            id: msg_id.to_string(),
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
            sender_name: u_name,
            text: "Convidou você para uma partida de Jogo da Velha.".to_string(),
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
            .entry(contact_id.clone())
            .or_default()
            .push(new_msg);

        // Simula o aceite do contato após 3 segundos
        let mut state = *self;
        let cid = contact_id;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            // Verifica se o jogo ainda está no estado de convite pendente
            let is_pending = {
                let games = state.game_states.read();
                games.get(&cid).map(|g| !g.accepted).unwrap_or(false)
            };

            if is_pending {
                // Aceita o convite
                {
                    let mut games = state.game_states.write();
                    if let Some(game) = games.get_mut(&cid) {
                        game.accepted = true;
                        game.active = true;
                    }
                }

                // Mensagem do contato aceitando
                let msg_id = state.message_counter();
                *state.message_counter.write() += 1;
                let now = chrono::Local::now().format("%H:%M:%S").to_string();
                let partner_name = state
                    .contacts()
                    .iter()
                    .find(|c| c.id == cid)
                    .map(|c| c.display_name.clone())
                    .unwrap_or_else(|| "Contato".to_string());

                let new_msg = Message {
                    id: msg_id.to_string(),
                    conversation_id: cid.clone(),
                    sender_id: cid.clone(),
                    sender_name: partner_name,
                    text: "Aceitou o desafio de Jogo da Velha! O jogo começou.".to_string(),
                    timestamp: now,
                    is_nudge: false,
                    font_color: "#2e6930".to_string(),
                    font_family: "Segoe UI".to_string(),
                    is_wink: None,
                    file_transfer: None,
                    is_game_invite: false,
                };
                state
                    .chat_messages
                    .write()
                    .entry(cid)
                    .or_default()
                    .push(new_msg);
            }
        });
    }

    pub fn accept_game_invite(&mut self, contact_id: String) {
        {
            let mut games = self.game_states.write();
            if let Some(game) = games.get_mut(&contact_id) {
                game.accepted = true;
                game.active = true;
            } else {
                let mut new_game = TicTacToe::new();
                new_game.accepted = true;
                games.insert(contact_id.clone(), new_game);
            }
        }

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_msg = Message {
            id: msg_id.to_string(),
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
            sender_name: self.user_name(),
            text: "Aceitou o desafio de Jogo da Velha! O jogo começou.".to_string(),
            timestamp: now,
            is_nudge: false,
            font_color: "#2e6930".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };
        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_msg);
    }

    pub fn reject_game_invite(&mut self, contact_id: String) {
        {
            self.game_states.write().remove(&contact_id);
        }

        let msg_id = self.message_counter();
        *self.message_counter.write() += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_msg = Message {
            id: msg_id.to_string(),
            conversation_id: contact_id.clone(),
            sender_id: "0".to_string(),
            sender_name: self.user_name(),
            text: "Recusou o desafio de Jogo da Velha.".to_string(),
            timestamp: now,
            is_nudge: false,
            font_color: "#e81123".to_string(),
            font_family: "Segoe UI".to_string(),
            is_wink: None,
            file_transfer: None,
            is_game_invite: false,
        };
        self.chat_messages
            .write()
            .entry(contact_id)
            .or_default()
            .push(new_msg);
    }

    pub fn make_game_move(&mut self, contact_id: String, cell_idx: usize) {
        let mut is_bot_turn = false;
        {
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
                is_bot_turn = true;
            }
        }

        if is_bot_turn {
            // Simula a jogada do oponente (Bot) após 1.2 segundos
            let cid = contact_id.clone();
            let mut state = *self;
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
                let mut games = state.game_states.write();
                if let Some(game) = games.get_mut(&cid) {
                    if game.active && game.turn == TicTacToeCell::O {
                        let empty_cells: Vec<usize> = game
                            .board
                            .iter()
                            .enumerate()
                            .filter(|(_, cell)| **cell == TicTacToeCell::Empty)
                            .map(|(idx, _)| idx)
                            .collect();

                        if !empty_cells.is_empty() {
                            // Pseudo-aleatório com o timestamp
                            let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(42) as u64;
                            let idx = (now % empty_cells.len() as u64) as usize;
                            let bot_cell = empty_cells[idx];

                            game.board[bot_cell] = TicTacToeCell::O;

                            if check_game_over(game) {
                                return;
                            }
                            game.turn = TicTacToeCell::X;
                        }
                    }
                }
            });
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
