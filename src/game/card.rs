use std::fmt;

use crate::{HAND_SIZE, NUM_PLAYERS};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    Nigiri,
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
            Card::Nigiri           => "Nigiri",
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
            if card.eq(&Card::Nigiri) {
                scores[*id] += 1;
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
    
}

#[allow(non_snake_case)]
pub fn score_Tea(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_GreenTeaIceCream(scores: &mut Vec<usize>, end_boards: &Vec<(usize, Vec<Card>)>) {

} 

// Counts of the number of search_card that appear in the cards vector
fn count_cards(search_card: Card, cards : &Vec<Card>) -> usize {
    cards.iter().filter(|card| search_card.eq(card)).count()
}