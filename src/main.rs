mod objects;
mod screen;
use objects::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
