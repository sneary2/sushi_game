use std::fmt;

use crate::NUM_PLAYERS;

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
pub fn score_Nigiri(scores: &mut Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {
    for (id, board) in end_boards {
        for card in board {
            if card.eq(&Card::Nigiri) {
                scores[*id] += 0;
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn score_Maki(scores: &mut Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {
    let maki_counts: Vec<(_, _)> = end_boards.iter().map(|(id, cards)| (id, count_cards(Card::Maki, cards))).collect();

    println!("maki_counts: {maki_counts:?}");

    // end_boards.sort_by(|a, b: &(usize, Vec<Card>)| b.1.iter().filter(|&card| card.eq(&Card::Maki)).count().cmp(&a.1.iter().filter(|&card| card.eq(&Card::Maki)).count()));
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

    // let first_player_id  = maki_counts[0].0;
    // let second_player_id = end_boards[1].0;
    // println!("Player with        most Maki is {first_player_id}");
    // println!("Player with second most Maki is {second_player_id}");
    // scores[first_player_id]  += 6;
    // scores[second_player_id] += 3;
}

#[allow(non_snake_case)]
pub fn score_Tempura(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_Sashimi(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_MisoSoup(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_Wasabi(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_Tea(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

#[allow(non_snake_case)]
pub fn score_GreenTeaIceCream(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

} 

fn count_cards(search_card: Card, cards : &Vec<Card>) -> usize {
    cards.iter().filter(|card| search_card.eq(card)).count()
}