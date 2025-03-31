use std::thread::{self};

use std::sync::{Arc, Mutex, Condvar};
// use std::thread;

mod game;
use game::card::Card;
use game::deck::Deck;
use game::deck::Setup;

fn main() {
    println!("Start Game!");

    let deck: Arc<Mutex<Deck>> = Arc::new(Mutex::new(Deck::new(Setup::MY_FIRST_MEAL)));
    let round: Arc<(Mutex<i32>, Condvar)> = Arc::new((Mutex::new(0), Condvar::new()));

    {
        deck.lock().unwrap().shuffle();
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for player_id in 0..=3 {

        let round: Arc<(Mutex<i32>, Condvar)> = round.clone();


        let hand: Vec<Card>;
        {
            hand = deck.lock().unwrap().draw(9);
        } 

        let field : Vec<Card> = Vec::with_capacity(9);

        handles.push(thread::spawn(move || game::player::create_player(player_id, round, hand, field)));
    }

    {
        let (turn_lock, cvar) = &*round;
        let mut turn: std::sync::MutexGuard<'_, i32> = turn_lock.lock().unwrap();
        *turn += 1;
        println!("Main: Updated turn to {turn}");
        cvar.notify_all();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
