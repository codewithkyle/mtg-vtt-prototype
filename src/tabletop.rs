use crate::card::Card;

pub struct Tabletop {
    pub cards: Vec<Card>,
}

impl Tabletop {
    pub fn new() -> Tabletop {
        let card = Card::new(
            [0.0, 0.0, 0.0],
            "",
            None,
            "It's a Good Day to Pie",
        );
        Self {
            cards: vec!(card),
        }
    }
}
