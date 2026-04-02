pub mod food;
pub mod game;
pub mod snake;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub const SCREEN_LENGTH: i32 = 50;
