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

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
pub fn staged() -> bool {
    std::env::args().any(|arg| arg == "--screenshot")
}

fn main() {
    start("Tessera", Box::new(TesseraGame::new()));
}
