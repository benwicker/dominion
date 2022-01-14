use rand::thread_rng;
use rand::seq::SliceRandom;
use super::card::Card;

pub struct Deck {
  pub cards: Vec<Card>
}

impl Deck {
  pub fn new() -> Deck {
    Deck {
      cards: Vec::new()
    }
  }

  pub fn new_pile(constructor: fn() -> Card, amount: i32) -> Deck {
    let mut deck = Deck::new();

    for _n in 0..amount {
      deck.cards.push(constructor());
    }

    deck
  }

  pub fn new_player_deck() -> Deck {
    let mut deck = Deck::new();

    for _n in 0..7 {
     deck.cards.push(Card::copper()); 
    }

    for _n in 0..3 {
      deck.cards.push(Card::estate());
    }

    deck.shuffle();

    deck
  }

  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng());
  }

  pub fn shuffle_discard_into_draw_pile(&mut self, discard_pile: &mut Deck) {
    self.cards = discard_pile.cards.clone();
    discard_pile.cards = Vec::new();
    self.shuffle();
  }

  pub fn cards_remaining(&self) -> usize {
    self.cards.len()
  }

  pub fn cards_available_to_draw(&self, discard_pile: &Deck) -> usize {
    self.cards_remaining() + discard_pile.cards_remaining()
  }

  pub fn draw_cards(&mut self, num_cards_to_draw: usize, optional_discard_pile: Option<&mut Deck>) -> Vec<Card> {
    let mut amount = num_cards_to_draw;
    if amount < 1 {
      panic!("cannot draw less than 1 card");
    }

    let mut drawn_cards: Vec<Card> = Vec::new();

    match optional_discard_pile {
      None => {
        if amount > self.cards_remaining() {
          panic!("You cannot draw that many cards from this pile");
        }
      }
      Some(discard_pile) => {
        if self.cards_available_to_draw(&discard_pile) < amount {
          panic!("There are not enough cards in this deck and it's discard to draw");
        }

        let cards_remaining = self.cards_remaining();
        
        if cards_remaining == 0 {
          self.shuffle_discard_into_draw_pile(discard_pile);
        } else if amount > cards_remaining {
          drawn_cards = self.draw_cards(cards_remaining, None);
          self.shuffle_discard_into_draw_pile(discard_pile);
          amount -= cards_remaining;
        }
      }
    }

    for _n in 0..amount {
      match self.cards.pop() {
        Some(card) => drawn_cards.push(card),
        None => panic!("Tried to draw a card but there wasn't one to draw")
      }
    }

    drawn_cards
  }

  pub fn add_cards(&mut self, cards: Vec<Card>) {
    for card in cards {
      self.cards.push(card);
    }
  }

  pub fn remove_card(&mut self, card: Card) -> Card {
      let index = self.cards.iter().position(|c| c.name == card.name).unwrap();
      self.cards.remove(index)
  }

  pub fn print(&self) {
    for card in &self.cards {
      println!("{}", card.name);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_new_pile() {
    let deck = Deck::new_pile(Card::copper, 10);

    assert_eq!(deck.cards_remaining(), 10);
  }

  #[test]
  fn shuffle_deck() {
    let mut deck = Deck::new_player_deck();

    deck.shuffle();

    assert_eq!(deck.cards_remaining(), 10);
  }

  #[test]
  fn draw_1_card_from_stack_of_10() {
    let mut deck = Deck::new_pile(Card::copper, 10);

    let drawn_cards = deck.draw_cards(1, None);

    assert_eq!(drawn_cards.len(), 1);
    assert_eq!(deck.cards_remaining(), 9);
  }

  #[test]
  fn draw_1_card_from_stack_of_1() {
    let mut deck = Deck::new_pile(Card::copper, 1);

    let drawn_cards = deck.draw_cards(1, None);

    assert_eq!(drawn_cards.len(), 1);
    assert_eq!(deck.cards_remaining(), 0);
  }

  #[test]
  fn draw_1_card_from_stack_of_10_with_discard_pile() {
    let mut deck = Deck::new_pile(Card::copper, 10);
    let mut discard_pile = Deck::new_pile(Card::copper, 1);

    let drawn_cards = deck.draw_cards(1, Some(&mut discard_pile));

    assert_eq!(drawn_cards.len(), 1);
    assert_eq!(discard_pile.cards_remaining(), 1);
    assert_eq!(deck.cards_remaining(), 9);
  }

  #[test]
  fn draw_2_card_from_stack_of_1_with_discard_pile_of_10() {
    let mut deck = Deck::new_pile(Card::copper, 1);
    let mut discard_pile = Deck::new_pile(Card::copper, 10);

    assert_eq!(deck.cards_available_to_draw(&discard_pile), 11);

    let drawn_cards = deck.draw_cards(2, Some(&mut discard_pile));

    assert_eq!(drawn_cards.len(), 2);
    assert_eq!(discard_pile.cards_remaining(), 0);
    assert_eq!(deck.cards_remaining(), 9);
    assert_eq!(deck.cards_available_to_draw(&discard_pile), 9);
  }
}