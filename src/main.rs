use std::thread::{self, current};

use rand::seq::SliceRandom; // For shuffling
use rand::thread_rng; // Random number generator

use std::sync::{Arc, Mutex, Condvar};
// use std::thread;

const NIGIRI_COUNT : u32 = 12;
const SUSHI_ROLLS_COUNT : u32 = 12;
const APPETIZERS_COUNT : u32 = 12;
const SPECIALS_COUNT : u32 = 12;
const DESSERTS_COUNT : u32 = 12;


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Card {
    Nigiri,
    Maki,
    Tempura,
    Sashimi,
    MisoSoup,
    Wasabi,
    Tea,
    GreeTeaIceCream
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum Setup {
    MY_FIRST_MEAL,
    SUSHI_GO,
    PARTY_SAMPLER,
    MASTER_MENU,
    POINTS_PLATTER,
    CUTTHROAT_COMBO,
    BIG_BANQUET,
    DINNER_FOR_TWO
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new(setup: Setup) -> Self {
        // let mut cards = Vec::new();
        let mut deck: Deck = Self{cards: Vec::new()};
        match setup {
            Setup::MY_FIRST_MEAL => {
                deck.add_card_to_game(Card::Nigiri);
                deck.add_card_to_game(Card::Maki);
                deck.add_card_to_game(Card::Tempura);
                deck.add_card_to_game(Card::Sashimi);
                deck.add_card_to_game(Card::MisoSoup);
                deck.add_card_to_game(Card::Wasabi);
                deck.add_card_to_game(Card::Tea);
                // deck.cards.push(Card::new(Card::GreeTeaIceCream);
            }
            _ => {
                println!("Not supported");
            }
        }
        deck
    }

    fn add_card_to_game(&mut self, card: Card) {
        match card {
            Card::Nigiri                                    => self.add_card(card, NIGIRI_COUNT     ),
            Card::Maki                                      => self.add_card(card, SUSHI_ROLLS_COUNT),
            Card::Tempura | Card::Sashimi | Card::MisoSoup  => self.add_card(card, APPETIZERS_COUNT ),
            Card::Wasabi | Card::Tea                        => self.add_card(card, SPECIALS_COUNT   ),
            Card::GreeTeaIceCream                           => self.add_card(card, DESSERTS_COUNT   ),
            // _                                               => println!("Card Not Supported Yet"),
        }
    }

    fn add_card(&mut self, card: Card, count: u32) {
        for _ in 0..count {
            self.cards.push(card.clone());
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    fn draw(&mut self, amount : usize) -> Vec<Card> {
        let mut hand = Vec::with_capacity(9);
        for _ in 0..amount {
            hand.push(self.cards.pop().unwrap());
        }
        return hand;
    }
}

fn main() {
    println!("Start Game!");
    
    // Deck::new(Setup::DINNER_FOR_TWO);

    let deck = Arc::new(Mutex::new(Deck::new(Setup::MY_FIRST_MEAL)));
    let round: Arc<(Mutex<i32>, Condvar)> = Arc::new((Mutex::new(0), Condvar::new()));

    // for card in &deck.cards {
    //     println!("Drew card: {:?}", card)
    // }

    // deck.shuffle();
    {
        deck.lock().unwrap().shuffle();
    }

    let mut handles = Vec::new();

    // for player_id in 0..1 {
    for player_id in 0..=3 {

        // let deck = deck.clone();
        let round = round.clone();


        // let hand: Vec<Card>;
        // {
        //     hand = deck.lock().unwrap().draw(9);
        // } 

        // let field : Vec<Card> = Vec::with_capacity(9);

        handles.push({thread::spawn(move || {
            
            let player = player_id;
            let (turn_lock, cvar) = &*round;
            // let mut curr_game_round = *turn_lock.lock().unwrap();
            let mut player_game_round = 1;

            // Wait for the round to start
            // while current_round != curr_game_round {

            let global_turn: i32 = *turn_lock.lock().unwrap();
            println!("Player {player}: Internal turn is {player_game_round}, Global turn is {global_turn}");

            let turn_mut = turn_lock.lock().unwrap();
            while player_game_round != *turn_mut {

                println!("Round is not correct, Player {player} will wait for the round to be updated");
                // Round is not correct
                // Wait for the next round
                // let turn_lock_mut = turn_lock.lock().unwrap();
                
                let turn_lock = cvar.wait(turn_mut).unwrap();

                // Update our current value for the round
                player_game_round = *turn_lock;

                println!("Player {player} has entered Turn {player_game_round}");
                return;
            }
            println!("Player {player} thread is exiting");
        })});
    }

    {
        let (turn_lock, cvar) = &*round;
        let mut turn = turn_lock.lock().unwrap();
        *turn += 1;
        println!("Main: Updated turn to {turn}");
        cvar.notify_all();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
