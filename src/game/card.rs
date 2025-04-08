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
