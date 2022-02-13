//TODO: Separate different parts of game to different files.
mod food;
mod snake;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const VIEW_WIDTH: i32 = 80;
    pub const VIEW_HEIGHT: i32 = 50;
    pub const FRAME_TIME: f32 = 50.0;
    pub use crate::food::*;
    pub use crate::snake::*;

    #[derive(Clone, Copy, Debug)]
    pub enum MovementDir {
        Up,
        Down,
        Right,
        Left,
        None,
    }
}

use crate::prelude::*;

struct State {
    snake: Snake,
    frame_time: f32,
    food: Food,
    input_buffer: [MovementDir; 3],
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);
        //store input buffer to call as soon as context
        //reaches frame time
        self.set_input_buffer(ctx);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_TIME {
            self.frame_time = 0.0;
            for movement in self.input_buffer {
                self.snake.move_snake(movement);
                if self.food.check_collision(self.snake.head_position) {
                    self.snake.add_tail();
                    self.snake.set_position_matrix();
                    self.food.place_food(self.snake.position_matrix);
                }
            }
            //get last movement and put it at front of input buffer
            self.input_buffer = [
                self.snake.movement_dir,
                MovementDir::None,
                MovementDir::None,
            ];
        }
        self.snake.render(ctx);
        self.food.render(ctx);
    }
}

impl State {
    fn new() -> Self {
        State {
            snake: Snake::new(),
            frame_time: 0.0,
            food: Food::new(),
            input_buffer: [MovementDir::Right, MovementDir::None, MovementDir::None],
        }
    }

    fn set_input_buffer(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left => {
                    for input in self.input_buffer.iter_mut() {
                        if matches!(input, MovementDir::None) {
                            *input = MovementDir::Left;
                            break;
                        } else if matches!(input, &mut MovementDir::Left) {
                            break;
                        }
                    }
                }
                VirtualKeyCode::Right => {
                    for input in self.input_buffer.iter_mut() {
                        if matches!(input, MovementDir::None) {
                            *input = MovementDir::Right;
                            break;
                        } else if matches!(input, &mut MovementDir::Right) {
                            break;
                        }
                    }
                }
                VirtualKeyCode::Up => {
                    for input in self.input_buffer.iter_mut() {
                        if matches!(input, MovementDir::None) {
                            *input = MovementDir::Up;
                            break;
                        } else if matches!(input, &mut MovementDir::Up) {
                            break;
                        }
                    }
                }
                VirtualKeyCode::Down => {
                    for input in self.input_buffer.iter_mut() {
                        if matches!(input, MovementDir::None) {
                            *input = MovementDir::Down;
                            break;
                        } else if matches!(input, &mut MovementDir::Down) {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    main_loop(context, State::new())
}
