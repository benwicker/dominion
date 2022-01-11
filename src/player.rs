use super::deck::Deck;

pub struct Player {
  pub available_coin: i32,
  pub available_actions: i32,
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
      draw_pile: Deck::new_player_deck(),
      discard_pile: Deck::new(),
      hand: Deck::new(),
    }
  }

  pub fn draw(&mut self, amount: usize) {
    self.hand.add_cards(self.draw_pile.draw_cards(amount, Some(&mut self.discard_pile)));
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