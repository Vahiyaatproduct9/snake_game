use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rand::RngExt;
#[derive(Clone, Copy, Debug)]
pub struct Food(pub i32, pub i32);
impl Food {
    pub fn new() -> Self {
        Self(
            rand::rng().random_range(0..SCREEN_HEIGHT - 3),
            rand::rng().random_range(0..SCREEN_WIDTH - 3),
        )
    }

    pub fn spawn(&mut self) {
        self.0 = rand::rng().random_range(0..SCREEN_HEIGHT - 3);
        self.1 = rand::rng().random_range(0..SCREEN_WIDTH - 3);
    }
}
