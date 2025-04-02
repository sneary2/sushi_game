use std::sync::{Arc, Barrier, Condvar, Mutex};

use crate::game::card::Card;

#[warn(unused_variables)]
pub fn create_player(player_id : usize, turn_barrier: Arc<Barrier>, mut hand : Vec<Card>, _field : Vec<Card>) {

    let mut turn_no: i32 = 0;
    println!("Player {player_id}:  Turn is {turn_no}");
    
    while hand.is_empty() == false {
        turn_barrier.wait();

        let card = hand.pop().unwrap();

        turn_no = turn_no+1;
        println!("Player {player_id} on round {turn_no}: Player {player_id} plays {card}")
    }

    println!("Player {player_id} thread is exiting");
}