mod objects;
mod screen;
use objects::game::Game;

pub const SCREEN_HEIGHT: i32 = 25;
pub const SCREEN_WIDTH: i32 = 25;
pub const SPEED: u64 = 1;
fn main() {
    Game::new().run().unwrap();
}
