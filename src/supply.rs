use super::deck::Deck;
use super::card::{Card, CardType};

#[derive(Clone)]
pub struct Supply {
  pub point_cards: Vec<Deck>,
  pub treasure_cards: Vec<Deck>,
  pub action_cards: Vec<Deck>,
  pub num_empty_piles: i32,
}

impl Supply {
  pub fn new_empty() -> Supply {
    Supply {
      point_cards: Vec::new(),
      treasure_cards: Vec::new(),
      action_cards: Vec::new(),
      num_empty_piles: 0,
    }
  }

  pub fn new() -> Supply {
    let mut supply = Supply::new_empty();

    // point cards
    supply.point_cards.push(Deck::new_pile(Card::estate, 8));

    // treasure cards
    supply.treasure_cards.push(Deck::new_pile(Card::copper, 10));

    // action cards
    supply.action_cards.push(Deck::new_pile(Card::militia, 10));

    return supply;
  }

  pub fn remove_card(&mut self, card: &Card) {
    match card.card_type {
      CardType::VictoryPoint(_) => Supply::remove_card_from_supply_type(card, &mut self.point_cards),
      CardType::Treasure(_) => Supply::remove_card_from_supply_type(card, &mut self.treasure_cards),
      CardType::Action(_) => Supply::remove_card_from_supply_type(card, &mut self.action_cards),
    } 
  }

  pub fn remove_card_from_supply_type(card: &Card, piles: &mut Vec<Deck>) {
    for pile in piles {
      if let Some(top_card) = pile.first() {
        if top_card.name == card.name {
          pile.remove_card(card.clone());
        }
      }
    }
  }
}