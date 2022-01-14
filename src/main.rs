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
    let mut game = game::Game::new();
    game.play();
}