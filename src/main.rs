#![warn(clippy::all, clippy::pedantic)]

use std::char::from_digit;

use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
// const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAX_VELOCITY: f32 = 3.0;
const MAX_ACCELERATION: f32 = 1.0;
const RESISTENCE: f32 = 0.3;

enum GameMode {
    Menu,
    Playing,
}

struct Person {
    origin: i32,
    destination: i32,
    wait_time: i32,
}

struct Elevator {
    floor: i32,
    capacity: i32,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    elevator: Elevator,
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            floor: 1,
            capacity: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(9, 9, RED, BLACK, to_cp437('#'));
        ctx.set(9, 10, RED, BLACK, to_cp437('#'));
        ctx.set(9, 11, RED, BLACK, to_cp437('#'));
        ctx.set(9, 12, RED, BLACK, to_cp437('#'));

        ctx.set(11, 9, RED, BLACK, to_cp437('#'));
        ctx.set(11, 10, RED, BLACK, to_cp437('#'));
        ctx.set(11, 11, RED, BLACK, to_cp437('#'));
        ctx.set(11, 12, RED, BLACK, to_cp437('#'));

        ctx.set(10, 9, RED, BLACK, to_cp437('#'));
        ctx.set(10, 12, RED, BLACK, to_cp437('#'));

        let floor_string = (self.floor as u32).to_string();

        for (i, c) in floor_string.chars().enumerate() {
            ctx.set(9 + i, 8, RED, BLACK, to_cp437(c));
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            elevator: Elevator::new(),
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
        self.elevator = Elevator::new();
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.elevator.render(ctx);
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
