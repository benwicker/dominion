use super::ui;

pub fn display() -> ui::GameActions {
  let options = vec![
    ui::InputOption::new("1".to_string(), "Start new game".to_string()),
    ui::InputOption::new("2".to_string(), "Quit".to_string())
  ];

  ui::clear_terminal();
  println!("Welcome to dominion cli. Select an action to get started");
  println!("");
  ui::print_options(options);
  println!("");
  let input = ui::get_user_input(options);
  process(input)
}

fn process(input: ui::InputOption) -> ui::GameActions {
  match input.cmd {
    input. => ui::GameActions::StartNewGame,
    2 => ui::GameActions::Quit,
    _ => panic!("Invalid input inside main menu: {}", input),
  }
}