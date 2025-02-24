use std::{collections::HashSet, hash::{Hash, Hasher}};

use crate::card::Card;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub id: u32,
    pub hand: HashSet<Card>,
    pub points: u32,
    pub card_points: u32,
}

impl User {
    pub fn new(name: &str, id: u32) -> User {
        User {
            name: name.to_string(),
            id,
            hand: HashSet::new(),
            points: 0,
            card_points: 0,
        }
    }
}

// Implement PartialEq and Eq using only the `id` field
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

// Implement Hash using only the `id` field
impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
