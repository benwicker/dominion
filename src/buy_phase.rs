use super::player::Player;
use super::deck::Deck;
use super::supply::Supply;
use super::card::CardType;
use super::ui;

pub fn perform_buy_phase(player: &mut Player, supply: &mut Supply) {
  ui::clear();
  ui::print("--- BUY PHASE ---");
  
  if player.available_buys == 0 {
    ui::line_break();
    ui::print("No more available buys.");
    ui::press_any_key_to_continue();
    return;
  }

  ui::print("Choose a card to buy. Format:[cost, quantity remaining]");
  ui::line_break();
  ui::print(&format!("Available Coin: {}", player.available_coin));
  ui::print(&format!("Available Buys: {}", player.available_buys));
  ui::line_break();

  let mut options: Vec<ui::InputOption> = Vec::new();

  ui::print("-- Point Cards --");
  get_options_and_display_cards(&supply.point_cards, player.available_coin, &mut options);

  ui::line_break();
  ui::print("-- Treasure Cards --");
  get_options_and_display_cards(&supply.treasure_cards, player.available_coin, &mut options);

  ui::line_break();
  ui::print("-- Action Cards --");
  get_options_and_display_cards(&supply.action_cards, player.available_coin, &mut options);

  ui::line_break();
  ui::line_break();
  let skip_option = ui::InputOption::new(String::from("s"), String::from("Skip buy phase"), None, Some(ui::InputAction::Skip));
  ui::print_option(&skip_option);
  options.push(skip_option);

  let chosen_option = ui::get_input_option(&mut options);
  
  // skip action
  if let Some(_) = chosen_option.action { return; }

  if let Some(card) = chosen_option.card {
    if let CardType::VictoryPoint(amount) = card.card_type {
      player.vp += amount;
    }

    player.available_buys -= 1;
    player.available_coin -= card.cost;
    supply.remove_card(&card);
    player.gain_card(card);    
  }

  perform_buy_phase(player, supply);
}

pub fn get_options_and_display_cards(supply_piles: &Vec<Deck>, available_coin: i32, options: &mut Vec<ui::InputOption>) {
  for pile in supply_piles {
    if let Some(example_card) = pile.first() {
      let label = format!("[{}, {}] {}", example_card.cost, pile.cards_remaining(), example_card.to_string());
      
      if example_card.cost <= available_coin {
        let cmd = options.len().to_string();
        let option = ui::InputOption::new(cmd, label, Some(example_card.clone()), None);
        ui::print_option(&option);
        options.push(option);
      } else {
        ui::print(&format!("-: {}", label));
      }
    } else {
      continue;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_perform_buy_phase() {
    let mut player = Player::new();
    let mut supply = Supply::new();

    player.available_buys = 2;
    player.available_coin = 10;

    perform_buy_phase(&mut player, &mut supply);
  }
}