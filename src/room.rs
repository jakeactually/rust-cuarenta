use std::collections::HashSet;

use serde::Serialize;

use crate::card::Card;
use crate::user::User;


#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub deck: Vec<Card>,
    pub board: HashSet<Card>,
    pub current_player: User,
    pub active: bool,
    pub players: Vec<User>,
    pub turn: usize,
    pub dirty: bool,
    pub claim: HashSet<Card>,
    pub last_card: Card,
}

impl Room {
    pub fn new() -> Self {
        Room {
            deck: Vec::new(),
            board: HashSet::new(),
            current_player: User::new("", 0),
            active: false,
            players: Vec::new(),
            turn: 0,
            dirty: false,
            claim: HashSet::new(),
            last_card: Card::new(0, "", "", ""),
        }
    }

    pub fn player(&mut self, player_id: u32) -> &mut User {
        self.players
            .iter_mut()
            .find(|player| player.id == player_id)
            .unwrap()
    }

    pub fn update_player(&mut self) {
        self.current_player = self.players[self.turn % self.players.len()].clone();
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.update_player();
    }

    pub fn push(&mut self, user: User) -> &mut Self {
        self.players.push(user);
        self
    }

    pub fn includes(&self, id: u32) -> bool {
        self.players.contains(&User::new("", id))
    }
}
