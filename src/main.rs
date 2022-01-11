// mod main_menu;
// pub mod ui;
pub mod player;
pub mod card;
pub mod deck;
pub mod game;
pub mod action_phase;
pub mod ui;


fn main() {  
    // display main menu
    let mut quit = false;
    while !quit {
        // match main_menu::display() {
        //     ui::GameActions::Quit => quit = true,
        //     ui::GameActions::StartNewGame => {
        //         play_game();
        //     }
        // }
    }
}

fn play_game() {
    println!("Starting a new game");
}