use std::sync::{Arc, Mutex, Condvar};

use crate::game::card::Card;

#[warn(unused_variables)]
pub fn create_player(player_id : i32, round : Arc<(Mutex<i32>, Condvar)>, mut hand : Vec<Card>, _field : Vec<Card>) {


    let (turn_lock, _) = &*round;
    let global_turn: i32 = *turn_lock.lock().unwrap();
    println!("Player {player_id}: Global turn is {global_turn}");
    
    while hand.is_empty() == false {
        wait_for_next_round(round.clone(), player_id);

        let card = hand.pop().unwrap();

        let global_turn: i32 = *turn_lock.lock().unwrap();
        println!("Player {player_id} has entered round {global_turn}. Player {player_id} plays {card}")
    }
    

    println!("Player {player_id} thread is exiting");
}

fn wait_for_next_round(round : Arc<(Mutex<i32>, Condvar)>, player_id : i32) {
    let (turn_lock, cvar) = &*round;
    
    println!("Round is not correct, Player {player_id} will wait for the round to be updated");

    let turn_mut = turn_lock.lock().unwrap();
    let turn_lock = cvar.wait(turn_mut).unwrap();
    
    let player_game_round = *turn_lock;
    println!("Player {player_id} has entered Turn {player_game_round}");
}