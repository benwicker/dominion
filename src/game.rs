use super::player::Player;
use super::action_phase::ActionPhase;
use super::buy_phase as BuyPhase;
use super::supply::Supply;
use super::ui;

pub struct Game {
  player: Player,
  supply: Supply,
  turn: i32,
}

impl Game {
  pub fn new() -> Game {
    Game {
      player: Player::new(),
      supply: Supply::new(),
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
      BuyPhase::perform_buy_phase(&mut self.player, &mut self.supply);

      // CLEAN UP PHASE
      self.player.reset_stats();
      
      // SHOW DEBUG INFO
      ui::show_debug_info(&self.player);
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