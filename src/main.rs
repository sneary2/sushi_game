use std::thread::{self};

use std::sync::{Arc, Mutex, Barrier};

mod game;
use game::card::{score_GreenTeaIceCream, score_Maki, score_MisoSoup, score_Nigiri, score_Sashimi, score_Tea, score_Tempura, score_Wasabi, Card};
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

        let field : Vec<Card> = Vec::with_capacity(HAND_SIZE);

        let barrier_clone: Arc<Barrier> = Arc::clone(&turn_barrier);

        handles.push(thread::spawn(move || game::player::create_player(player_id, barrier_clone, hand, field)));
    }


    for round_no in 1..=9 {
        turn_barrier.wait();
        println!("=== Round {round_no} Start!");

        rotate_hands(&hands);

        turn_barrier.wait();
    }

    let mut end_boards: Vec<(usize, Vec<Card>)> = Vec::with_capacity(NUM_PLAYERS);
    for handle in handles {
        let (player_id, played_cards) = handle.join().unwrap();

        // println!("Player {player_id} played {played_cards:?}");

        end_boards.push((player_id, played_cards));
    }

    print_board(&end_boards);

    let mut scores: Vec<usize> = vec![0; 4];

    score_Nigiri(&mut scores, &end_boards);
    score_Maki(&mut scores, &end_boards);
    score_Tempura(&mut scores, &end_boards);
    score_Sashimi(&mut scores, &end_boards);
    score_MisoSoup(&mut scores, &end_boards);
    score_Wasabi(&mut scores, &end_boards);
    score_Tea(&mut scores, &end_boards);
    score_GreenTeaIceCream(&mut scores, &end_boards);

    let player_ids: Vec<_> = (0..4).collect();
    let mut player_scores: Vec<_> = player_ids.into_iter().zip(scores).collect();

    player_scores.sort_by(|p1,p2| p2.1.cmp(&p1.1));

    for (player_id, score) in player_scores {
        println!("Player {player_id} scored {score}");
    }

}

fn rotate_hands(hands : &Vec<Arc<Mutex<Vec<Card>>>>) {
    let mut temp_hands : Vec<Vec<Card>> = hands.iter().map(|hand: &Arc<Mutex<Vec<Card>>>| hand.lock().unwrap().clone()).collect();

    temp_hands.rotate_right(1);

    for (i, hand) in hands.iter().enumerate() {
        *hand.lock().unwrap() = temp_hands[i].clone();
    }
}

fn print_board(end_boards: &Vec<(usize, Vec<Card>)>) {
    println!("+---------------------------------------------------+");

    print!("| ");
    for (id, _) in end_boards {
        let card = format!("Player {}", id);
        print!("{card:10}");
        print!(" | ");
    }
    println!("");

    for row in 0..HAND_SIZE {
        println!("+---------------------------------------------------+");
        print_table_row(end_boards, row);
    }
    println!("+---------------------------------------------------+");
}

fn print_table_row(end_boards: &Vec<(usize, Vec<Card>)>, row: usize) {
    print!("| ");
    for (_, cards) in end_boards {
        let card = format!("{:10}", &cards[row].to_string());
        print!("{card}");
        print!(" | ");
    }
    println!("");
}