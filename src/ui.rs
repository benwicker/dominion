use std::io;
use super::card::Card;

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
    println!("{}: {}", option.cmd, option.label);
  }
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