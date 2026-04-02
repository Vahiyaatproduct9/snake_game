use crate::objects::{Snake, food};
#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub height: i32,
    pub width: i32,
}

impl Screen {
    pub fn square(length: i32) -> Self {
        Self {
            height: length,
            width: length,
        }
    }
    pub fn build(&self, snake: &Snake, food: &food::Food) -> String {
        let mut res = Vec::<String>::new();
        let mut baseline = String::new();
        for _ in 0..self.height {
            baseline.push('#');
        }
        res.push(baseline.clone());
        for h in 0..self.height - 2 {
            let mut local_res = String::new();
            for w in 0..self.width - 2 {
                let food::Food(food_x, food_y) = food;
                if snake.body.contains(&(w, h)) {
                    local_res.push('@');
                } else if *food_x == w && *food_y == h {
                    local_res.push('*');
                } else {
                    local_res.push(' ');
                }
            }
            res.push(format!("#{}#", local_res));
        }
        res.push(baseline);
        res.join("\r\n")
    }
}
