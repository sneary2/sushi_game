use std::thread::{self};

use std::sync::{Arc, Mutex, Barrier};
// use std::thread;

mod game;
use game::card::Card;
use game::deck::Deck;
use game::deck::Setup;

const NUM_PLAYERS : usize = 4;
const HAND_SIZE: usize = 9;

fn main() {
    println!("Start Game!");

    let deck: Arc<Mutex<Deck>> = Arc::new(Mutex::new(Deck::new(Setup::MY_FIRST_MEAL)));
    let turn_barrier: Arc<Barrier> =  Arc::new(Barrier::new(NUM_PLAYERS + 1)); // 4 players

    {
        deck.lock().unwrap().shuffle();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for player_id in 0..NUM_PLAYERS {

        let hand: Vec<Card>;
        {
            hand = deck.lock().unwrap().draw(HAND_SIZE);
        } 

        let field : Vec<Card> = Vec::with_capacity(9);

        let barrier_clone = Arc::clone(&turn_barrier);

        handles.push(thread::spawn(move || game::player::create_player(player_id, barrier_clone, hand, field)));
    }


    for round_no in 1..=9 {
        // println!("=== Round {round_no} Start!");
        turn_barrier.wait();      
    }

    for handle in handles {
        handle.join().unwrap();
    }

}
