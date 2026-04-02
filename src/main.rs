mod objects;
mod screen;
use objects::game::Game;

fn main() {
    Game::new().run().unwrap();
}
