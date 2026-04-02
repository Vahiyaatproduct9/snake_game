use std::{
    io::Result,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::RngExt;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH, SPEED,
    objects::{self, Direction, food::Food, snake::Snake},
    screen::Screen,
};

#[derive(Eq, PartialEq)]
pub enum GameState {
    Over,
    Playing,
}

pub struct Game {
    pub food: Food,
    pub game: GameState,
    pub snake: Snake,
    pub score: u32,
    pub screen: Screen,
}

impl Game {
    pub fn build(&self) {
        let screen = self.screen.build(&self.snake, &self.food);
        println!("{}", screen);
        println!("\rScore: {}", self.score);
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let tick_rate = Duration::from_millis(500 / SPEED);
        let mut last_tick = Instant::now();
        let poll_rate = if 500 / SPEED < 50 { 500 / SPEED } else { 50 };
        loop {
            print!("{:?}", self.food);
            if event::poll(Duration::from_millis(poll_rate)).unwrap()
                && let Event::Key(key) = event::read().unwrap()
            {
                if key.code == KeyCode::Char('q') {
                    break;
                };
                match key.code {
                    KeyCode::Up => {
                        if self.snake.direction != Direction::Down {
                            self.snake.direction = objects::Direction::Up;
                        }
                    }
                    KeyCode::Down => {
                        if self.snake.direction != Direction::Up {
                            self.snake.direction = objects::Direction::Down;
                        }
                    }
                    KeyCode::Left => {
                        if self.snake.direction != Direction::Right {
                            self.snake.direction = objects::Direction::Left;
                        }
                    }
                    KeyCode::Right => {
                        if self.snake.direction != Direction::Left {
                            self.snake.direction = objects::Direction::Right;
                        }
                    }
                    _ => (),
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.snake.move_forward();
                self.check_collision();
                self.ate_food();
                self.build();
                last_tick = Instant::now();
            }

            if self.game == GameState::Over {
                println!("\rGAME OVER! YOU SCORED: {}\n\r", self.score);
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    pub fn new() -> Self {
        let random_x = rand::rng().random_range(0..(SCREEN_WIDTH / 2));
        let random_y = rand::rng().random_range(0..(SCREEN_HEIGHT / 2));
        let snake_pos_x = random_x + (SCREEN_WIDTH / 4);
        let snake_pos_y = random_y + (SCREEN_HEIGHT / 4);
        Self {
            food: Food::new(),
            game: GameState::Playing,
            snake: Snake::new(Some((snake_pos_x, snake_pos_y))),
            score: 0,
            screen: Screen::new(SCREEN_HEIGHT, SCREEN_WIDTH),
        }
    }

    pub fn check_collision(&mut self) {
        let head = self.snake.body.front().unwrap();
        let (x, y) = *head;

        let Screen {
            height: scrh,
            width: scrw,
        } = self.screen;

        if x >= scrw - 2 || x < 0 || y >= scrh - 2 || y < 0 {
            self.game = GameState::Over
        }

        if self.snake.body.iter().skip(1).any(|pos| pos == head) {
            self.game = GameState::Over
        }
    }

    pub fn ate_food(&mut self) {
        let Food(food_pos_x, food_pos_y) = self.food;
        let head_pos = self.snake.body.front().unwrap();
        let (head_pos_x, head_pos_y) = *head_pos;
        if food_pos_x == head_pos_x && food_pos_y == head_pos_y {
            self.snake.grow();
            self.increase_score();
            self.food.spawn();
        }
    }

    fn increase_score(&mut self) {
        self.score += 1;
    }
}
