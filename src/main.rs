use blitzkit::start;

mod bag;
mod board;
mod input;
mod layout;
mod piece;
mod state;
mod system;
mod tessera_game;
mod util;

use tessera_game::TesseraGame;

fn main() {
    start("Tessera", Box::new(TesseraGame::new()));
}
