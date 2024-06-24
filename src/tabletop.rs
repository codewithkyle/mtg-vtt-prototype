use crate::card::Card;

pub struct Tabletop {
    cards: Vec<Card>,
}

impl Tabletop {
    pub fn new() -> Tabletop {
        Self {
            cards: vec!(),
        }
    }
}
