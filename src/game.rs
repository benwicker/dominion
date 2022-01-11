use super::player::Player;

pub struct Game {
  player: Player,
  turn: i32,
}

impl Game {
  pub fn new() {
    
  }

  pub fn play(&mut self) {
    // start by displaying all available cards
    // ...press any key to continue

    while !self.is_game_over() {
      
      // START TURN
      self.turn += 1;
      self.player.draw(5);

      // ACTION PHASE
      // play all non action cards
      // if player doesn't have an available action points or action cards in hand
        // display action phase menu with press any key to continue to buy phase
      // while player has available aciton points and action cards in hand
        // display action phase menu with played cards and available
        // get input
        // if action card
          // play
        // if skip to buy phase, break

      // BUY PHASE
      // while user has buys display buy menu
        // get user input
        // if buy, add card to discard pile (add vp points if applicable)
        // if end turn, go to clean up phase
      // if user doesn't have buys, display menu with press any key to continue

      // CLEAN UP PHASE
      // discard all played cards
      // check for win condition

    }
  }

  pub fn is_game_over(&self) -> bool {
    true
  }
}