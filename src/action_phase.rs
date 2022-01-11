use super::ui;
use super::card::Card;
use super::card::CardType;
use super::deck::Deck;

pub struct ActionPhase {
  action_points: i32,
  played_cards: Vec<Card>,
  coin: i32,
}

impl ActionPhase {
  pub fn new() -> ActionPhase {
    ActionPhase {
      action_points: 1,
      coin: 0,
      played_cards: Vec::new(),
    }
  }

  pub fn display_menu(&self, hand: &Deck) {
    let hand_clone = hand.cards.clone();
    let (action_cards, non_action_cards): (Vec<_>, Vec<_>) = hand_clone.into_iter().partition(|c| matches!(c.card_type, CardType::Action(_)));

    ui::clear();
    ui::print("--- ACTION PHASE ---");
    
    self.print_state();
    self.print_played_cards();
    self.print_non_action_cards(non_action_cards);

    if self.action_points == 0 {
      ui::line_break();
      ui::print("You don't have any remaining actions.");
      ui::line_break();
      ui::press_any_key_to_continue();
      return;
    }

    if action_cards.len() == 0 {
      ui::line_break();
      ui::print("You don't have any more action cards to play.");
      ui::line_break();
      ui::press_any_key_to_continue();
      return;
    }

    ui::line_break();
    ui::print("Choose a card to play:");
    let options = self.get_playable_options(action_cards);
    ui::print_options(options);
  }

  fn print_played_cards(&self) {
    ui::line_break();
    ui::print("Played cards:");
    for card in &self.played_cards {
      let card_string = String::from("- ") + &card.to_string();
      ui::print(&card_string);
    }
  }

  fn print_state(&self) {
    ui::print(&format!("Actions remaining: {}", self.action_points));
    ui::print(&format!("Coin: {}", self.coin));
  }

  fn print_non_action_cards(&self, cards: Vec<Card>) {
    if cards.len() == 0 {
      return;
    }

    ui::line_break();
    ui::print("Non action cards in hand:");
    for card in &cards {
      ui::print(&format!("- {}", card.to_string()))
    }
  }

  fn get_playable_options(&self, cards: Vec<Card>) -> Vec<ui::InputOption> {
    let mut options: Vec<ui::InputOption> = Vec::new();
    for (i, card) in cards.into_iter().enumerate() {
      options.push(ui::InputOption::new(i.to_string(), card.to_string(), Some(card), None));
    }

    options.push(ui::InputOption::new(String::from("s"), String::from("Skip to buy phase"), None, Some(ui::InputAction::Skip)));

    options
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_display_menu() {
    let mut hand = Deck::new();
    hand.add_cards(vec![
      // Card::militia(),
    ]);

    let played_card = Card::copper();
    let action_phase = ActionPhase {
      action_points: 1,
      played_cards: vec![played_card],
      coin: 0,
    };

    action_phase.display_menu(&hand);

    assert_eq!(true, true);
  }
}
