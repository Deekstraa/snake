use crate::prelude::*;

pub struct Snake {
    pub movement_dir: MovementDir,
    pub head_position: Point,
    velocity: i32,
    pub tail: Vec<SnakeTail>,
    pub position_matrix: [[bool; VIEW_HEIGHT as usize]; VIEW_WIDTH as usize],
}

impl Snake {
    pub fn new() -> Self {
        Self {
            movement_dir: MovementDir::Right,
            head_position: Point::new(20, 25),
            velocity: 1,
            tail: Vec::new(),
            position_matrix: [[true; VIEW_HEIGHT as usize]; VIEW_WIDTH as usize],
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

        for tail_piece in self.tail.iter() {
            ctx.set(
                tail_piece.position.x,
                tail_piece.position.y,
                YELLOW,
                BLACK,
                to_cp437('#'),
            );
        }
    }

    pub fn move_snake(&mut self, input: MovementDir) {
        if Snake::is_allowed_movement(self.movement_dir, input) {
            match input {
                MovementDir::Down => {
                    self.shift_tail();
                    self.head_position.y += self.velocity;
                    self.movement_dir = input;
                }
                MovementDir::Up => {
                    self.shift_tail();
                    self.head_position.y -= self.velocity;
                    self.movement_dir = input;
                }
                MovementDir::Left => {
                    self.shift_tail();
                    self.head_position.x -= self.velocity;
                    self.movement_dir = input;
                }
                MovementDir::Right => {
                    self.shift_tail();
                    self.head_position.x += self.velocity;
                    self.movement_dir = input;
                }
                MovementDir::None => {
                    //if movement direction is none keep current movement
                }
            }
        }

        if self.collide() {
            //TODO: Clear tail on death
            self.head_position = Point::new(20, 25);
            self.movement_dir = MovementDir::Right;
        }
    }

    pub fn add_tail(&mut self) {
        let mut spawn_position = Point::zero();
        let mut mov_dir = MovementDir::None;

        if let Some(tail_end) = self.tail.last() {
            spawn_position = Snake::get_new_tail_pos(tail_end.movement_dir, tail_end.position);
            mov_dir = tail_end.movement_dir;
        } else {
            spawn_position = Snake::get_new_tail_pos(self.movement_dir, self.head_position);
            mov_dir = self.movement_dir;
        }

        self.tail
            .push(SnakeTail::new(mov_dir, spawn_position, self.velocity));
    }

    pub fn set_position_matrix(&mut self) {
        //keep track of the position matrix as the snake is updated.
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
        //TODO: Check for collision with own tail
        false
    }

    fn get_new_tail_pos(dir: MovementDir, tail_pos: Point) -> Point {
        let mut pos: Point = Point::zero();
        match dir {
            MovementDir::Down => {
                pos = Point::new(tail_pos.x, tail_pos.y - 1);
            }
            MovementDir::Up => {
                pos = Point::new(tail_pos.x, tail_pos.y + 1);
            }
            MovementDir::Left => {
                pos = Point::new(tail_pos.x + 1, tail_pos.y);
            }
            MovementDir::Right => {
                pos = Point::new(tail_pos.x - 1, tail_pos.y);
            }
            MovementDir::None => {}
        }
        pos
    }

    fn shift_tail(&mut self) {
        if self.tail.is_empty() {
            return;
        } else {
            let mut next_pos = Point::new(self.head_position.x, self.head_position.y);
            let mut next_dir = self.movement_dir;
            let mut _pos = Point::zero();
            let mut _dir = MovementDir::None;

            for tail_piece in self.tail.iter_mut() {
                _pos = Point::new(tail_piece.position.x, tail_piece.position.y);
                _dir = tail_piece.movement_dir;
                tail_piece.position = Point::new(next_pos.x, next_pos.y);
                tail_piece.movement_dir = next_dir;
                next_pos = Point::new(_pos.x, _pos.y);
                next_dir = _dir;
            }
        }
    }
}

#[derive(Debug)]
pub struct SnakeTail {
    pub movement_dir: MovementDir,
    pub position: Point,
    velocity: i32,
}

impl SnakeTail {
    fn new(movement_dir: MovementDir, position: Point, velocity: i32) -> Self {
        Self {
            movement_dir,
            position,
            velocity,
        }
    }
}
