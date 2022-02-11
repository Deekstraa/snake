use crate::prelude::*;

pub struct Snake {
    pub movement_dir: MovementDir,
    head_position: Point,
    velocity: i32,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            movement_dir: MovementDir::Right,
            head_position: Point::new(20, 25),
            velocity: 1,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.head_position.x,
            self.head_position.y,
            YELLOW,
            BLACK,
            to_cp437('#'),
        );
    }

    pub fn move_snake(&mut self, input_buffer: [MovementDir; 3]) {
        for input in input_buffer.iter() {
            if Snake::is_allowed_movement(self.movement_dir, *input) {
                match input {
                    MovementDir::Down => {
                        self.change_direction(*input);
                        self.head_position.y += self.velocity;
                    }
                    MovementDir::Up => {
                        self.change_direction(*input);
                        self.head_position.y -= self.velocity;
                    }
                    MovementDir::Left => {
                        self.change_direction(*input);
                        self.head_position.x -= self.velocity;
                    }
                    MovementDir::Right => {
                        self.change_direction(*input);
                        self.head_position.x += self.velocity;
                    }
                    MovementDir::None => {
                        //if movement direction is none keep current movement
                    }
                }
            }

            if self.collide() {
                self.head_position = Point::new(20, 25);
                self.movement_dir = MovementDir::Right;
            }
        }
    }

    fn change_direction(&mut self, new_direction: MovementDir) {
        if Snake::is_allowed_movement(self.movement_dir, new_direction) {
            self.movement_dir = new_direction;
        }
    }

    fn is_allowed_movement(dir1: MovementDir, dir2: MovementDir) -> bool {
        if matches!(dir1, MovementDir::Left) && matches!(dir2, MovementDir::Right)
            || matches!(dir2, MovementDir::Left) && matches!(dir1, MovementDir::Right)
        {
            return false;
        }
        if matches!(dir1, MovementDir::Up) && matches!(dir2, MovementDir::Down)
            || matches!(dir2, MovementDir::Up) && matches!(dir1, MovementDir::Down)
        {
            return false;
        }
        true
    }

    fn collide(&self) -> bool {
        if self.head_position.x >= VIEW_WIDTH
            || self.head_position.x < 0
            || self.head_position.y < 0
            || self.head_position.y >= VIEW_HEIGHT
        {
            return true;
        }

        false
    }
}
