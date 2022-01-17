use std::io;
use super::card::Card;
use super::player::Player;

// pub fn get_user_input(options: Vec<InputOption>) -> GameActions {
//   print!("Type a number then press enter: ");
//   io::Write::flush(&mut io::stdout()).expect("flush failed!");

  // let mut input_text = String::new();
  // io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
  // let trimmed = input_text.trim();
//   // return game action of matching option
// }

// pub fn print_options(options: Vec<InputOption>) {
//   for option in options {
//     println!("{}: {}", option.cmd, option.label);
//   }
// }

pub fn clear() {
  print!("\x1B[2J\x1B[1;1H");
}

pub fn print(text: &str) {
  println!("{}", text);
}

pub fn line_break() {
  println!("");
}

pub fn print_options(options: &Vec<InputOption>) {
  for option in options {
    print_option(option);
  }
}

pub fn print_option(option: &InputOption) {
  println!("{}: {}", option.cmd, option.label);
}

pub fn get_input_option(options: &mut Vec<InputOption>) -> InputOption {
  line_break();
  print!("Type an option then press enter: ");
  io::Write::flush(&mut io::stdout()).expect("flush failed!");

  let line = read_trimmed_line();
  let result = options.iter().position(|o| o.cmd == line);

  match result {
    None => {
      println!("Invalid input. '{}' is not an option", line);
      return get_input_option(options)
    },
    Some(index) => {
      return options.swap_remove(index);
    }
  }
}

pub fn press_any_key_to_continue() {
  println!("Press any key to continue...");
  read_trimmed_line();
}

fn read_trimmed_line() -> String {
  let mut input_text = String::new();
  io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
  input_text.trim().to_string()
}

pub fn show_debug_info(player: &Player) {
  clear();
  println!("DEBUG INFO");
  line_break();
  line_break();
  println!("--- PLAYER STATS ---");
  println!("available coin: {}", player.available_coin);
  println!("available buys: {}", player.available_buys);
  println!("available actions: {}", player.available_actions);
  println!("available vp: {}", player.vp);
  line_break();
  println!("--- PLAYER DRAW PILE");
  player.draw_pile.print();
  line_break();
  println!("--- PLAYER DISCARD PILE");
  player.discard_pile.print();
  line_break();
  println!("--- PLAYER HAND ---");
  player.hand.print();
  line_break();
  press_any_key_to_continue();
}

#[derive(Clone)]
pub struct InputOption {
  cmd: String,
  label: String,
  pub card: Option<Card>,
  pub action: Option<InputAction>
}

impl InputOption {
  pub fn new(cmd: String, label: String, card: Option<Card>, action: Option<InputAction>) -> InputOption {
    InputOption {
      cmd,
      label,
      card,
      action,
    }
  }
}

#[derive(Clone)]
pub enum InputAction {
  Skip
}