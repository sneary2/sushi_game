use std::fmt;

use crate::{HAND_SIZE, NUM_PLAYERS};


#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    Nigiri_1,
    Nigiri_2,
    Nigiri_3,
    Maki,
    Tempura,
    Sashimi,
    MisoSoup,
    Wasabi,
    Tea,
    GreenTeaIceCream
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_name = match self {
            Card::Nigiri_1         => "Nigiri (1)",
            Card::Nigiri_2         => "Nigiri (2)",
            Card::Nigiri_3         => "Nigiri (3)",
            Card::Maki             => "Maki",
            Card::Tempura          => "Tempura",
            Card::Sashimi          => "Sashimi",
            Card::MisoSoup         => "Miso Soup",
            Card::Wasabi           => "Wasabi",
            Card::Tea              => "Tea",
            Card::GreenTeaIceCream => "Green Tea Ice Cream",
        };
        write!(f, "{}", card_name)
    }
}


#[allow(non_snake_case)]
pub fn score_Nigiri(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    for (id, board) in end_boards {
        for card in board {
            if card.eq(&Card::Nigiri_1) {
                scores[*id] += 1;
            }
            if card.eq(&Card::Nigiri_2) {
                scores[*id] += 2;
            }
            if card.eq(&Card::Nigiri_3) {
                scores[*id] += 3;
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn score_Maki(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    let maki_counts: Vec<(&usize, usize)> = end_boards.iter().map(|(id, cards)| (id, count_cards(Card::Maki, cards))).collect();

    println!("maki_counts: {maki_counts:?}");

    let most_maki = maki_counts[0].1;
    let mut idx = 0;
    while idx < NUM_PLAYERS && maki_counts[idx].1 == most_maki {
        scores[*maki_counts[idx].0]  += 6;
        idx += 1;
    }
    if idx < NUM_PLAYERS{
        let second_most_maki = maki_counts[idx].1;
        while idx < NUM_PLAYERS && maki_counts[idx].1 == second_most_maki {
            scores[*maki_counts[idx].0] += 3;
            idx += 1;
        }
    }
}

#[allow(non_snake_case)]
pub fn score_Tempura(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    let tempura_counts: Vec<(&usize, usize)> = end_boards.iter().map(|(id, cards)| (id, count_cards(Card::Tempura, cards))).collect();

    for (id, tempura_count) in tempura_counts {
        scores[*id] += (tempura_count / 2) * 5;
    }
}

#[allow(non_snake_case)]
pub fn score_Sashimi(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    let sashimi_counts: Vec<(&usize, usize)> = end_boards.iter().map(|(id, cards)| (id, count_cards(Card::Sashimi, cards))).collect();

    for (id, tempura_count) in sashimi_counts {
        scores[*id] += (tempura_count / 3) * 10;
    }
}

#[allow(non_snake_case)]
pub fn score_MisoSoup(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    'round_loop: for round in 0..HAND_SIZE {
        let mut miso_played: bool = false;
        let mut id_won_miso: usize = 4096;
        for (player_id, cards) in end_boards {
            if cards[round].eq(&Card::MisoSoup) {
                if miso_played {
                    // Miso Soup has already been played, no points awarded, moving to next loop
                    continue 'round_loop;
                }
                id_won_miso = *player_id;
                miso_played = true;
            }
        }
        if miso_played == true {
            scores[id_won_miso] += 3;
        }
    }
}

#[allow(non_snake_case)]
pub fn score_Wasabi(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    for (player_id, card) in end_boards {
        let mut has_wasabi = false;
        for card in card {
            if card.eq(&Card::Wasabi) {
                has_wasabi = true;
            }
            if has_wasabi {
                if card.eq(&Card::Nigiri_1) {
                    scores[*player_id] += 1 * 2;
                    has_wasabi = false;
                } else if card.eq(&Card::Nigiri_2) {
                    scores[*player_id] += 2 * 2;
                    has_wasabi = false;
                } else if card.eq(&Card::Nigiri_3) {
                    scores[*player_id] += 3 * 2;
                    has_wasabi = false;
                }
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn score_Tea(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    for (player_id, cards) in end_boards {
        let num_tea: usize = count_cards(Card::Tea, cards);
        if num_tea > 0 {
            let mut colour_count: Vec<usize> = vec![0; NUM_COLOURS];
            for card in cards {
                let card_colour: usize = card.get_colour();
                colour_count[card_colour] += 1;
            }
            let score: usize = num_tea * colour_count.iter().max().unwrap();
            scores[*player_id] += score;
        }
    }
}

#[allow(non_snake_case)]
pub fn score_GreenTeaIceCream(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {
    let GreenTeaIceCream_counts: Vec<(&usize, usize)> = end_boards.iter().map(|(id, cards)| (id, count_cards(Card::GreenTeaIceCream, cards))).collect();

    for (id, GreenTeaIceCream_count) in GreenTeaIceCream_counts {
        scores[*id] += (GreenTeaIceCream_count / 4) * 12;
    }
} 

// Counts of the number of search_card that appear in the cards vector
fn count_cards(search_card: Card, cards : &Vec<Card>) -> usize {
    cards.iter().filter(|card| search_card.eq(card)).count()
}


const YELLOW    : usize = 0;
const RED       : usize = 1;
const PURPLE    : usize = 2;
const GREEN     : usize = 3;
const TURQUOISE : usize = 4;
const BROWN     : usize = 5;
const BLUE      : usize = 6;

// const COLOURS: [usize; NUM_COLOURS] = [YELLOW, RED, PURPLE, GREEN, TURQUOISE, BROWN, BLUE];
const NUM_COLOURS: usize = 7;

impl Card {
    fn get_colour(&self) -> usize {
        return match self {
            Card::Nigiri_1         => YELLOW,
            Card::Nigiri_2         => YELLOW,
            Card::Nigiri_3         => YELLOW,
            Card::Maki             => RED,
            Card::Tempura          => PURPLE,
            Card::Sashimi          => GREEN,
            Card::MisoSoup         => TURQUOISE,
            Card::Wasabi           => YELLOW,
            Card::Tea              => BROWN,
            Card::GreenTeaIceCream => BLUE,
        };
    }
}
