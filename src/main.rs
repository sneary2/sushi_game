fn main() {
    println!("Hello, world!");
    let deck : Deck = Deck::new(Setup::DINNER_FOR_TWO);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    name: String,
    id: u32,
}

impl Card {
    fn new(name: &str, id: u32) -> Self {
        Self{
            name: name.to_string(), 
            id
        }
    }
}

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
        let mut cards = Vec::new();
        match setup {
            Setup::MY_FIRST_MEAL => {
                cards = Vec::new();

                cards.push(Card::new("Nigiri", 1));
            }
            _ => {
                println!("Not supported");
            }
        }
        Self{cards}
    }
}