#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;
use more_asserts::*;

const FRAME_DURATION: f32 = 75.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAX_VELOCITY: f32 = 3.0;
const MAX_ACCELERATION: f32 = 1.0;
const RESISTENCE: f32 = 0.3;

enum GameMode {
    Menu,
    Playing,
}

struct Player {
    x: i32,
    y: i32,
    direction_x: f32,
    direction_y: f32,
    velocity: f32,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            direction_x: 0.0,
            direction_y: 0.0,
            velocity: 0.0,
        }
    }

    fn accelerate(&mut self, amount: f32) {
        if amount > MAX_ACCELERATION {
            self.velocity += MAX_ACCELERATION;
        } else {
            self.velocity += amount;
        }

        if self.velocity > MAX_VELOCITY {
            self.velocity = MAX_VELOCITY;
        }
    }

    fn point(&mut self, x: f32, y: f32) {
        self.direction_x += x;
        self.direction_y += y;

        let magnitude = (self.direction_x.powf(2.0) + self.direction_x.powf(2.0))
            .sqrt()
            .abs();

        if magnitude == 0.0 {
            return;
        }

        self.direction_x /= magnitude;
        self.direction_y /= magnitude;
    }

    fn tick(&mut self) {
        self.x += (self.velocity * self.direction_x).round() as i32;
        self.y += (self.velocity * self.direction_y).round() as i32;

        if self.velocity > 0.0 {
            self.velocity -= RESISTENCE;
        } else {
            self.velocity = 0.0;
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('#'))
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Peace");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.start_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn start_game(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        let mut x_dir: f32 = 0.0;
        let mut y_dir: f32 = 0.0;

        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if let Some(VirtualKeyCode::W) = ctx.key {
            y_dir = -1.0;
        }
        if let Some(VirtualKeyCode::A) = ctx.key {
            x_dir = -1.0;
        }
        if let Some(VirtualKeyCode::S) = ctx.key {
            y_dir = 1.0;
        }
        if let Some(VirtualKeyCode::D) = ctx.key {
            x_dir = 1.0;
        }

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            if x_dir != 0.0 || y_dir != 0.0 {
                self.player.accelerate(1.0);
            }
            self.player.point(x_dir, y_dir);
            self.player.tick();
        }

        self.player.render(ctx);

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::Menu;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_gt;

    use crate::Player;

    #[test]
    fn player_doesnt_move() {
        let mut player = Player::new(0, 0);
        player.tick();

        assert_eq!(player.x, 0);
        assert_eq!(player.y, 0);
    }

    #[test]
    fn player_move_right() {
        let mut player = Player::new(0, 0);

        player.point(1.0, 0.0);
        player.accelerate(1.0);
        player.tick();

        assert_gt!(player.x, 0);
    }

    #[test]
    fn player_move_up() {
        let mut player = Player::new(0, 0);

        player.point(0.0, 1.0);
        player.accelerate(1.0);
        player.tick();

        assert_gt!(player.y, 0);
    }
}
