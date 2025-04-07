use std::thread::{self};

use std::sync::{Arc, Mutex, Barrier};

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

    let mut handles: Vec<thread::JoinHandle<(usize, Vec<Card>)>> = Vec::with_capacity(NUM_PLAYERS);
    let mut hands : Vec<Arc<Mutex<Vec<Card>>>> = Vec::with_capacity(NUM_PLAYERS);

    for player_id in 0..NUM_PLAYERS {

        let hand : Arc<Mutex<Vec<Card>>>;
        {
            hand = Arc::new(Mutex::new(deck.lock().unwrap().draw(HAND_SIZE)));
            hands.push(hand.clone());
        } 

        let field : Vec<Card> = Vec::with_capacity(9);

        let barrier_clone: Arc<Barrier> = Arc::clone(&turn_barrier);

        handles.push(thread::spawn(move || game::player::create_player(player_id, barrier_clone, hand, field)));
    }


    for round_no in 1..=9 {
        turn_barrier.wait();
        println!("=== Round {round_no} Start!");

        rotate_hands(&hands);

        turn_barrier.wait();
    }

    let mut end_board: Vec<(usize, Vec<Card>)> = Vec::with_capacity(NUM_PLAYERS);
    for handle in handles {
        let (player_id, played_cards) = handle.join().unwrap();

        println!("Player {player_id} played {played_cards:?}");

        end_board.push((player_id, played_cards));
    }


}

fn rotate_hands(hands : &Vec<Arc<Mutex<Vec<Card>>>>) {
    let mut temp_hands : Vec<Vec<Card>> = hands.iter().map(|hand: &Arc<Mutex<Vec<Card>>>| hand.lock().unwrap().clone()).collect();

    temp_hands.rotate_right(1);

    for (i, hand) in hands.iter().enumerate() {
        *hand.lock().unwrap() = temp_hands[i].clone();
    }
}
