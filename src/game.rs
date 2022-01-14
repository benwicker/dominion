use super::player::Player;
use super::action_phase::ActionPhase;

pub struct Game {
  player: Player,
  turn: i32,
}

impl Game {
  pub fn new() -> Game {
    Game {
      player: Player::new(),
      turn: 0,
    }
  }

  pub fn play(&mut self) {
    // start by displaying all available cards
    // ...press any key to continue

    while !self.is_game_over() {
      
      // START TURN
      self.turn += 1;
      self.player.draw(5);

      // ACTION PHASE
      let mut action_phase = ActionPhase::new();
      action_phase.perform_action_phase(&mut self.player);

      // CLEAN UP
      action_phase.discard_played_cards(&mut self.player.discard_pile);
      self.player.play_non_action_cards_and_discard_unplayed_action_cards();

      // BUY PHASE
      // while user has buys display buy menu
        // get user input
        // if buy, add card to discard pile (add vp points if applicable)
        // if end turn, go to clean up phase
      // if user doesn't have buys, display menu with press any key to continue

      // CLEAN UP PHASE
      // check for win condition

    }
  }

  pub fn is_game_over(&self) -> bool {
    self.turn > 5
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_play_game() {
    let mut game = Game::new();

    game.play();
  }
}