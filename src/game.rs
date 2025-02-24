use crate::card::Card;
use crate::room::Room;
use crate::uno::{room_and_player, Cuarenta, CuarentaSocket};
use crate::user::User;
use actix_session::Session;
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::web::Json;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Serialize)]
struct Game {
    pub room: Room,
    pub player: User,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Turn {
    action: String,
    hand: Option<Card>,
    board: Vec<Card>,
}

// Helper functions
fn shuffle(room: &mut Room) {
    if room.deck.is_empty() {
        let mut rng = thread_rng();
        room.deck = Card::all();
        room.deck.shuffle(&mut rng);
        room.board = HashSet::new();
    }

    for player in &mut room.players {
        let hand: Vec<Card> = room.deck.drain(0..5).collect();
        player.hand = hand.into_iter().collect();
    }
    room.active = true;
}

async fn notify(subscribers: &mut Vec<CuarentaSocket>, room_id: u32) {
    for subscription in subscribers.iter_mut() {
        let _ = subscription.session.text("update").await;
    }
}

// Handlers
pub async fn play(
    data: web::Data<Mutex<Cuarenta>>,
    req: HttpRequest,
    session: Session,
) -> impl Responder {
    let mut context = data.lock().unwrap();
    let (room_id, player_id) = room_and_player(req, session);
    let maybe_room = context.rooms.get_mut(&room_id);

    if maybe_room.is_none() {
        return Err(actix_web::error::ErrorNotFound("Room does not exist"));
    }

    let room = maybe_room.unwrap();
    let players_amount = room.players.len();
    let is_room_valid = players_amount == 2 || players_amount == 4;

    if !is_room_valid {
        return Err(ErrorBadRequest("There must be 2 or 4 players"))
    }

    if !room.active {
        shuffle(room);
    }

    room.update_player();

    Ok(web::Json(Game {
        room: room.clone(),
        player: room.player(player_id).clone(),
    }))
}

pub async fn turn(
    data: web::Data<Mutex<Cuarenta>>,
    req: HttpRequest,
    session: Session,
    turn: web::Json<Turn>,
) -> impl Responder {
    let mut context = data.lock().unwrap();
    let (room_id, player_id) = room_and_player(req, session);
    let room = context.rooms.get_mut(&room_id).unwrap();

    if !room.active {
        return Err(ErrorBadRequest("Room isn't active"));
    }

    if room.current_player.id != player_id {
        return Err(ErrorUnauthorized("It's not your turn"));
    }
    
    let res: Result<String, String> = match turn.action.as_str() {
        "sum" => sum(room, &turn),
        "pass" => pass(room),
        "claim" => claim(room, &turn),
        _ => return Err(ErrorBadRequest("Invalid action")),
    };
    
    if let Some(hand) = &turn.hand {
        room.last_card = hand.clone();
    }

    if turn.action != "pass" {
        let current_player = room.current_player.clone();
        let player = room.player(player_id);
        player.hand = current_player.hand.clone();
        player.points = current_player.points.clone();
        player.card_points = current_player.card_points.clone();
    }

    notify(context.subscribers.get_mut(&room_id).unwrap(), 0).await;

    res.map_err(ErrorBadRequest)
}

fn sum(room: &mut Room, turn: &Turn) -> Result<String, String> {
    if room.dirty {
        return Err("You already threw a card".to_string());
    }

    room.dirty = true;
    let hand = &turn.hand;

    if turn.board.is_empty() {
        if let Some(card) = hand {
            room.current_player.hand.remove(card);
            room.board.insert(card.clone());
        }

    } else {
        if hand.as_ref().map(|c| c.value()) != Some(turn.board.iter().map(|c| c.value()).sum()) {
            return Err("Those cards don't add up".to_string());
        }

        if let Some(card) = hand {
            room.current_player.hand.remove(card);
            room.board.retain(|c| !turn.board.contains(c));
            room.current_player.card_points += turn.board.len() as u32 + 1;

            if card.value() == room.last_card.value() {
                room.current_player.points += 2;
            }
        }

        if room.board.is_empty() {
            room.current_player.points += 2;
        }
    }

    room.claim.clear();

    if let Some(card) = hand {
        for i in (card.chain_value() + 1).. {
            let next_cards: Vec<Card> = room.board.iter().filter(|c| c.chain_value() == i).cloned().collect();
            if next_cards.is_empty() {
                break;
            }
            room.claim.extend(next_cards);
        }
    }

    Ok("Sum successful".to_string())
}

fn pass(room: &mut Room) -> Result<String, String> {
    if !room.dirty {
        return Err("You haven't thrown a card".to_string());
    }

    room.next_turn();
    room.dirty = false;

    if room.players.iter().all(|p| p.hand.is_empty()) {
        shuffle(room);
    }

    Ok("Pass successful".to_string())
}

fn claim(room: &mut Room, turn: &Turn) -> Result<String, String> {
    if room.claim.is_empty() {
        return Err("There is nothing to claim".to_string());
    }

    if !room.claim.is_superset(&turn.board.iter().cloned().collect()) {
        return Err("You can't claim those cards".to_string());
    }

    room.current_player.card_points += room.claim.len() as u32;
    room.claim.clear();
    room.board.retain(|c| !turn.board.contains(c));

    if room.board.is_empty() {
        room.current_player.points += 2;
    }

    Ok("Claim successful".to_string())
}
