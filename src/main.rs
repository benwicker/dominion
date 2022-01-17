// mod main_menu;
// pub mod ui;
pub mod player;
pub mod card;
pub mod deck;
pub mod game;
pub mod action_phase;
pub mod ui;
pub mod buy_phase;
pub mod supply;


fn main() {  
    // display main menu
    let mut game = game::Game::new();
    game.play();
}