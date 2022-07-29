#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAX_VELOCITY: f32 = 5.0;
const ACCELERATION: f32 = 1.0;
const RESISTENCE: f32 = 0.7;

enum GameMode {
    Menu,
    Playing,
}

struct Player {
    x: i32,
    y: i32,
    velocity_x: f32,
    velocity_y: f32,
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
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    fn accelerate(&mut self, x: f32, y: f32) {
        if x != 0.0 {
            self.velocity_x += x;
        }

        if y != 0.0 {
            self.velocity_y += y;
        }
    }

    fn tick(&mut self) {
        self.x += self.velocity_x as i32;
        self.y += self.velocity_y as i32;
        // self.y += self.velocity as i32;
        // self.x += 1;
        // if self.y < 0 {
        //     self.y = 0;
        // }
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
            self.player.accelerate(x_dir, y_dir);
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
