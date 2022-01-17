use super::deck::Deck;
use super::card::{Card, CardType};

pub struct Player {
  pub available_coin: i32,
  pub available_actions: i32,
  pub available_buys: i32,
  pub vp: i32,
  pub draw_pile: Deck,
  pub discard_pile: Deck,
  pub hand: Deck
}

impl Player {
  pub fn new() -> Player {
    Player {
      vp: 0,
      available_coin: 0,
      available_actions: 1,
      available_buys :1,
      draw_pile: Deck::new_player_deck(),
      discard_pile: Deck::new(),
      hand: Deck::new(),
    }
  }

  pub fn draw(&mut self, amount: usize) {
    self.hand.add_cards(self.draw_pile.draw_cards(amount, Some(&mut self.discard_pile)));
    self.sort();
  }

  pub fn gain_card(&mut self, card: Card) {
    self.discard_pile.add_cards(vec![card]);
  }

  pub fn sort(&mut self) {
    self.hand.cards.sort_by(|a, b| { a.name.cmp(&b.name) });
  }

  pub fn remove_card_from_hand(&mut self, card: Card) -> Card {
    self.hand.remove_card(card)
  }

  pub fn play_non_action_cards_and_discard_unplayed_action_cards(&mut self) {
    let hand = self.hand.cards.clone();
    let (action_cards, non_action_cards): (Vec<_>, Vec<_>) = hand.into_iter().partition(|c| matches!(c.card_type, CardType::Action(_)));

    for card in &non_action_cards {
      card.play(self);
    }

    self.discard_pile.add_cards(non_action_cards);
    self.discard_pile.add_cards(action_cards);

    self.hand = Deck::new();
  }

  pub fn reset_stats(&mut self) {
    self.available_actions = 1;
    self.available_coin = 0;
    self.available_buys = 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn draw_cards() {
    let mut player = Player::new();

    player.draw(4);

    assert_eq!(player.hand.cards_remaining(), 4);
    assert_eq!(player.draw_pile.cards_remaining(), 6);
  }
}