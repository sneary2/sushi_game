use std::fmt;

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

pub fn score_Nigiri(scores: &mut Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {
    for (id, board) in end_boards {
        for card in board {
            if card.eq(&Card::Nigiri) {
                scores[*id] += 1;
            }
        }
    }
}

pub fn score_Maki(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_Tempura(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_Sashimi(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_MisoSoup(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_Wasabi(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_Tea(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

}

pub fn score_GreenTeaIceCream(scores: &Vec<u32>, end_boards: &Vec<(usize, Vec<Card>)>) {

} 