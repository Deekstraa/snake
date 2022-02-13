//randomly place a '.' in the console
//if the snake collides with the dot
//then pick it up and store the value
//OR trigger an event.

use crate::prelude::*;

pub struct Food {
    pub position: Point,
}

impl Food {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        Self {
            position: Point::new(rng.range(0, VIEW_WIDTH), rng.range(0, VIEW_HEIGHT)),
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }

    pub fn place_food(
        &mut self,
        position_matrix: [[bool; VIEW_HEIGHT as usize]; VIEW_WIDTH as usize],
    ) {
        let allowed_list = Food::get_allowed_list(position_matrix);
        let mut rng = RandomNumberGenerator::new();
        let index = rng.range(0, allowed_list.len());

        self.position = allowed_list[index];
    }

    pub fn check_collision(&self, snake_position: Point) -> bool {
        self.position == snake_position
    }

    fn get_allowed_list(
        position_matrix: [[bool; VIEW_HEIGHT as usize]; VIEW_WIDTH as usize],
    ) -> Vec<Point> {
        let mut allowed_list: Vec<Point> = Vec::new();

        for (x, i) in position_matrix.iter().enumerate() {
            for (y, val) in i.iter().enumerate() {
                if *val {
                    allowed_list.push(Point::new(x, y));
                }
            }
        }

        allowed_list
    }
}
