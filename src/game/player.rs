use std::sync::{Arc, Barrier, Mutex};

use crate::game::card::Card;

#[warn(unused_variables)]
pub fn create_player(player_id : usize, turn_barrier: Arc<Barrier>, hand : Arc<Mutex<Vec<Card>>>, _field : Vec<Card>) {

    let mut turn_no: i32 = 0;
    let mut played_cards: Vec<Card> = Vec::with_capacity(9);
    println!("Player {player_id}:  Turn is {turn_no}");
    
    while hand.lock().unwrap().is_empty() == false {
        turn_barrier.wait();
        turn_barrier.wait();

        let mut player_hand: std::sync::MutexGuard<'_, Vec<Card>> = hand.lock().unwrap();

        let card = player_hand.pop().unwrap();
        println!("Player {player_id} on round {turn_no}: Player {player_id} plays {card}");
        
        played_cards.push(card);
        turn_no = turn_no+1;
    }

    println!("Player {player_id} thread is exiting");
}