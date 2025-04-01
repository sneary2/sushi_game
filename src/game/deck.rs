use crate::game::card::Card;

use rand::seq::SliceRandom; // For shuffling
use rand::thread_rng; // Random number generator

const NIGIRI_COUNT : u32 = 12;
const SUSHI_ROLLS_COUNT : u32 = 12;
const APPETIZERS_COUNT : u32 = 12;
const SPECIALS_COUNT : u32 = 12;
const DESSERTS_COUNT : u32 = 12;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Setup {
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
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(setup: Setup) -> Self {
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

    pub fn add_card_to_game(&mut self, card: Card) {
        match card {
            Card::Nigiri                                    => self.add_card(card, NIGIRI_COUNT     ),
            Card::Maki                                      => self.add_card(card, SUSHI_ROLLS_COUNT),
            Card::Tempura | Card::Sashimi | Card::MisoSoup  => self.add_card(card, APPETIZERS_COUNT ),
            Card::Wasabi | Card::Tea                        => self.add_card(card, SPECIALS_COUNT   ),
            Card::GreenTeaIceCream                           => self.add_card(card, DESSERTS_COUNT   ),
            // _                                               => println!("Card Not Supported Yet"),
        }
    }

    pub fn add_card(&mut self, card: Card, count: u32) {
        for _ in 0..count {
            self.cards.push(card.clone());
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    pub fn draw(&mut self, amount : usize) -> Vec<Card> {
        let mut hand = Vec::with_capacity(9);
        for _ in 0..amount {
            hand.push(self.cards.pop().unwrap());
        }
        return hand;
    }
}