use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    id: u64,
    name: String,
    number: String,
    sign: String,
}

impl Card {
    pub fn new(id: u64, name: &str, number: &str, sign: &str) -> Self {
        Card { id, name: name.to_string(), number: number.to_string(), sign: sign.to_string() }
    }

    pub fn value(&self) -> u8 {
        VALUES.get(&self.number).cloned().unwrap_or(0)
    }

    pub fn chain_value(&self) -> u8 {
        CHAIN_VALUES.get(&self.number).cloned().unwrap_or(0)
    }

    pub fn all() -> Vec<Card> {
        let numbers = vec!["A", "2", "3", "4", "5", "6", "7", "J", "Q", "K"];
        let signs = vec!["C", "D", "H", "S"];

        numbers.iter().enumerate().flat_map(|(i, &number)| {
            signs.iter().enumerate().map(move |(j, &sign)| {
                let id = (i * 4 + j) as u64;
                let name = format!("{}{}", number, sign);
                Card::new(id, &name, number, sign)
            })
        }).collect()
    }
}

// Define static global values using lazy_static
lazy_static! {
    static ref VALUES: HashMap<String, u8> = {
        let mut map = HashMap::new();
        map.insert("A".to_string(), 1);
        map.insert("2".to_string(), 2);
        map.insert("3".to_string(), 3);
        map.insert("4".to_string(), 4);
        map.insert("5".to_string(), 5);
        map.insert("6".to_string(), 6);
        map.insert("7".to_string(), 7);
        map.insert("J".to_string(), 11);
        map.insert("Q".to_string(), 12);
        map.insert("K".to_string(), 13);
        map
    };

    static ref CHAIN_VALUES: HashMap<String, u8> = {
        let mut map = HashMap::new();
        map.insert("A".to_string(), 1);
        map.insert("2".to_string(), 2);
        map.insert("3".to_string(), 3);
        map.insert("4".to_string(), 4);
        map.insert("5".to_string(), 5);
        map.insert("6".to_string(), 6);
        map.insert("7".to_string(), 7);
        map.insert("J".to_string(), 8);
        map.insert("Q".to_string(), 9);
        map.insert("K".to_string(), 10);
        map
    };
}
