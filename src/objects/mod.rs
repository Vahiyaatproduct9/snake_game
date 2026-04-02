use std::collections::VecDeque;
pub mod game;
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Snake {
    pub body: VecDeque<(u32, u32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(position: Option<(u32, u32)>) -> Self {
        let mut body: VecDeque<(u32, u32)> = VecDeque::new();
        const DEFAULT_LENGTH: u32 = 3;
        for i in 0..DEFAULT_LENGTH {
            body.push_back((i, 0));
        }
        if position.is_none() {
            return Snake {
                body,
                direction: Direction::Right,
            };
        }
        let (posx, posy) = position.unwrap();

        Snake {
            body: body
                .iter_mut()
                .map(|(x, y)| (*x + posx, *y + posy))
                .collect(),
            direction: Direction::Left,
        }
    }
    pub fn grow(&mut self) {
        let (x, y) = self.body.front().unwrap();
        match self.direction {
            Direction::Up => self.body.push_front((*x, *y - 1)),
            Direction::Left => self.body.push_front((*x - 1, *y)),
            Direction::Down => self.body.push_front((*x, *y + 1)),
            Direction::Right => self.body.push_front((*x + 1, *y)),
        }
    }

    pub fn move_forward(&mut self) {
        self.grow();
        self.body.pop_back();
    }
}
