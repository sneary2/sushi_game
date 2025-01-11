use rand::seq::SliceRandom; // For shuffling
use rand::thread_rng; // Random number generator

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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Card {
//     name: CardType,
//     id: u32,
// }

// impl Card {
//     fn new(name: CardType, id: u32) -> Self {
//         Self{
//             name,
//             id
//         }
//     }
// }

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
                deck.add_card(Card::Nigiri, 5);
                deck.add_card(Card::Maki, 5);
                deck.add_card(Card::Tempura, 5);
                deck.add_card(Card::Sashimi, 5);
                deck.add_card(Card::MisoSoup, 5);
                deck.add_card(Card::Wasabi, 5);
                deck.add_card(Card::Tea, 5);
                // deck.cards.push(Card::new(Card::GreeTeaIceCream, 1);
            }
            _ => {
                println!("Not supported");
            }
        }
        deck
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
}

fn main() {
    println!("Start Game!");
    
    Deck::new(Setup::DINNER_FOR_TWO);

    let mut deck : Deck = Deck::new(Setup::MY_FIRST_MEAL);

    for card in &deck.cards {
        println!("Drew card: {:?}", card)
    }

    deck.shuffle();

    


}
