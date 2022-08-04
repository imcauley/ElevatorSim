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

type FloorIndex = i32;
type ElevatorIndex = i32;

#[derive(Clone, Copy)]
struct Person {
    origin: FloorIndex,
    destination: FloorIndex,
    elevator: ElevatorIndex,
    wait_time: i32,
}

#[derive(Clone)]
struct Floor {
    number: FloorIndex,
}

#[derive(Clone)]
struct Elevator {
    number: ElevatorIndex,
    current_floor: FloorIndex,
    destination_floor: FloorIndex,
    capacity: i32,
    max_capacity: i32,
}

struct State {
    mode: GameMode,
    frame_time: f32,
}

impl Person {
    fn tick(&mut self) {}

    fn can_enter_elevator(self, elevator: Elevator) -> bool {
        if self.origin != elevator.current_floor {
            return false;
        }

        if !elevator.has_capacity() {
            return false;
        }

        true
    }
}

impl Floor {
    // fn available_elevator(self) -> Option<Elevator> {
    //     for elevator in self.elevators {
    //         if elevator.has_capacity() {
    //             Some(elevator)
    //         }
    //     }
    // }
}

impl Elevator {
    fn has_capacity(self) -> bool {
        self.max_capacity < self.capacity
    }
}

fn add_people_to_elevator<'a>(people: &mut Vec<Person>, elevator: &mut Elevator) {
    for person in people {
        if person.can_enter_elevator(elevator.clone()) {
            elevator.capacity += 1;
        } else {
            break;
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
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
