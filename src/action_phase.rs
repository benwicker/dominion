use super::ui;
use super::card::Card;
use super::card::CardType;
use super::player::Player;
use super::deck::Deck;

pub struct ActionPhase {
  played_cards: Vec<Card>,
}

impl ActionPhase {
  pub fn new() -> ActionPhase {
    ActionPhase {
      played_cards: Vec::new(),
    }
  }

  pub fn perform_action_phase(&mut self, player: &mut Player) {
    let hand_clone = player.hand.cards.clone();
    let (action_cards, non_action_cards): (Vec<_>, Vec<_>) = hand_clone.into_iter().partition(|c| matches!(c.card_type, CardType::Action(_)));

    ui::clear();
    ui::print("--- ACTION PHASE ---");
    
    self.print_state(&player);
    self.print_played_cards();
    self.print_non_action_cards(non_action_cards);

    if player.available_actions == 0 {
      self.print_action_cards(action_cards);
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
    let mut options = self.get_playable_options(action_cards);
    ui::print_options(&options);
    let chosen_option = ui::get_input_option(&mut options);

    // skip action
    if let Some(_) = chosen_option.action { return; }

    if let Some(card) = chosen_option.card {
      card.play(player);
      self.played_cards.push(player.remove_card_from_hand(card));
      player.available_actions -= 1;
    } else {
      panic!("Invalid option chosen");
    }

    // recurse until skips or no available actions or action cards
    self.perform_action_phase(player);
  }

  pub fn discard_played_cards(&self, discard_pile: &mut Deck) {
    discard_pile.add_cards(self.played_cards.clone());
  }

  fn print_played_cards(&self) {
    ui::line_break();
    ui::print("Played cards:");
    for card in &self.played_cards {
      let card_string = String::from("- ") + &card.to_string();
      ui::print(&card_string);
    }
  }

  fn print_state(&self, player: &Player) {
    ui::print(&format!("Actions remaining: {}", player.available_actions));
    ui::print(&format!("Coin: {}", player.available_coin));
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

  fn print_action_cards(&self, cards: Vec<Card>) {
    if cards.len() == 0 {
      return;
    }

    ui::line_break();
    ui::print("Action cards in hand:");
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
  fn test_perform_action_phase() {
    let hand = Deck {
      cards: vec![
        Card::copper(),
        Card::copper(),
        Card::estate(),
        Card::militia(),
        Card::militia(),
      ]
    };
    
    let mut player = Player {
      vp: 0,
      available_coin: 0,
      available_actions: 1,
      available_buys: 1,
      draw_pile: Deck::new_player_deck(),
      discard_pile: Deck::new(),
      hand: hand,
    };

    let mut action_phase = ActionPhase::new();
    action_phase.perform_action_phase(&mut player);
  }
}
