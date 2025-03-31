use std::sync::{Arc, Mutex, Condvar};

use crate::game::card::Card;

pub fn create_player(player_id : i32, round : Arc<(Mutex<i32>, Condvar)>, hand : Vec<Card>, field : Vec<Card>) {
    let (turn_lock, cvar) = &*round;
    // let mut curr_game_round = *turn_lock.lock().unwrap();
    let mut player_game_round = 1;

    // Wait for the round to start
    // while current_round != curr_game_round {

    let global_turn: i32 = *turn_lock.lock().unwrap();
    println!("Player {player_id}: Internal turn is {player_game_round}, Global turn is {global_turn}");

    let turn_mut = turn_lock.lock().unwrap();
    while player_game_round != *turn_mut {

        println!("Round is not correct, Player {player_id} will wait for the round to be updated");
        // Round is not correct
        // Wait for the next round
        // let turn_lock_mut = turn_lock.lock().unwrap();
        
        let turn_lock = cvar.wait(turn_mut).unwrap();

        // Update our current value for the round
        player_game_round = *turn_lock;

        println!("Player {player_id} has entered Turn {player_game_round}");
        return;
    }
    println!("Player {player_id} thread is exiting");
}