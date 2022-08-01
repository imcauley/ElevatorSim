#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, fmt};

use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
// const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

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

struct Person {
    origin: i32,
    destination: i32,
    wait_time: i32,
}

struct Floor {
    elevator: Option<Elevator>,
    people: Vec<Person>,
}

struct Elevator {
    floor: i32,
    capacity: i32,
    people: Vec<Person>,
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
            people: Vec::new(),
        }
    }

    fn add_person_to_elevator(&mut self, person: Person) -> Result<(), ElevatorFull> {
        if self.elevator_is_full() {
            return Err(ElevatorFull {});
        }

        self.people.push(person);

        Ok(())
    }

    fn elevator_is_full(&mut self) -> bool {
        self.people.len() as i32 >= self.capacity
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

impl Floor {
    fn transfer_people_to_elevator(&mut self, elevator: Elevator) {
        while !self.people.is_empty() {}
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
