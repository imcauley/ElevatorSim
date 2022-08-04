#![warn(clippy::all, clippy::pedantic)]

use rand::Rng;
use std::{error::Error, fmt};

use bracket_lib::prelude::*;

struct ElevatorFull;

impl Error for ElevatorFull {}

impl fmt::Display for ElevatorFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl fmt::Debug for ElevatorFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

enum GameMode {
    Menu,
    Playing,
}

#[derive(Clone, Copy)]
struct Person {
    origin: &Floor,
    destination: &Floor,
    elevator: Option<&Elevator>,
    wait_time: i32,
}

#[derive(Clone)]
struct Floor {
    number: i32,
    elevators: Vec<&Elevator>,
}

#[derive(Clone)]
struct Elevator {
    current_floor: i32,
    destination_floor: i32,
    capacity: i32,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    building: Building,
}

impl Person {
    fn tick(&mut self) {
        match self.elevator {
            None => {}
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            building: Building::new(10, 1),
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
        self.frame_time = 0.0;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.building.tick();
        // self.elevator.render(ctx);
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
