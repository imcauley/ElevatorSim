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

#[derive(Clone)]
struct Person {
    origin: i32,
    destination: i32,
    wait_time: i32,
}

struct Floor {
    number: i32,
    elevator: Option<Elevator>,
    people: Vec<Person>,
}

struct Elevator {
    current_floor: i32,
    destination_floor: i32,
    destionaton_queue: Vec<i32>,
    capacity: i32,
    people: Vec<Person>,
}

struct Building {
    floors: Vec<Floor>,
    elevators: Vec<Elevator>,
    people: Vec<Person>,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    elevator: Elevator,
}

impl Person {
    fn new(origin: i32, destination: i32) -> Self {
        Person {
            origin: origin,
            destination: destination,
            wait_time: 0,
        }
    }

    fn tick(&mut self) {
        self.wait_time += 1
    }
}

impl Building {
    fn new(floors: i32, elevators: i32) -> Self {
        let mut building = Building {
            floors: Vec::new(),
            elevators: Vec::new(),
            people: Vec::new(),
        };

        for index in 0..floors {
            building.floors.push(Floor::new(index + 1));
        }

        for _ in 0..elevators {
            building.elevators.push(Elevator::new());
        }

        building
    }

    fn tick(&mut self) {
        for floor in self.floors.iter() {
            // empty people from elevator to floor
            // add people from floor to elevatorå
            // add people from rooms to floor
        }
    }

    fn generate_people_going_out(&mut self) -> Vec<Person> {
        let mut people: Vec<Person> = Vec::new();
        for _ in 0..19 {
            people.push(Person::new(self.random_floor(), 1))
        }

        people
    }

    fn generate_people_coming_in(&mut self) -> Vec<Person> {
        let mut people: Vec<Person> = Vec::new();
        for _ in 0..19 {
            people.push(Person::new(1, self.random_floor()))
        }

        people
    }

    fn random_floor(&mut self) -> i32 {
        let max_floor = self.floors.len() as i32;
        let mut rng = rand::thread_rng();

        rng.gen_range(1..max_floor)
    }

    fn elevator_at_floor(&mut self, number: i32) -> Option<&mut Elevator> {
        for elevator in self.elevators.iter_mut() {
            if elevator.current_floor == number {
                return Some(elevator);
            }
        }

        return None;
    }
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            current_floor: 1,
            destination_floor: 1,
            destionaton_queue: Vec::new(),
            capacity: 12,
            people: Vec::new(),
        }
    }

    fn tick(&mut self) {
        if self.current_floor < self.destination_floor {
            self.current_floor += 1;
        } else if self.current_floor > self.destination_floor {
            self.current_floor -= 1;
        } else {
            match self.destionaton_queue.pop() {
                Some(floor) => self.destination_floor = floor,
                None => {}
            }
        }
    }

    fn add_destination_floor(&mut self, floor: i32) {
        self.destionaton_queue.push(floor);
    }

    fn transfer_people_to_elevator(&mut self, people: Vec<Person>) -> Vec<Person> {
        while !people.is_empty() {
            for (index, person) in people.iter().enumerate() {
                match self.add_person_to_elevator(person.clone()) {
                    Ok(()) => {}
                    Err(_) => return people[index..].to_vec(),
                };
            }
        }

        Vec::from([])
    }

    fn add_person_to_elevator(&mut self, person: Person) -> Result<(), ElevatorFull> {
        if self.is_full() {
            return Err(ElevatorFull {});
        }

        self.people.push(person);

        Ok(())
    }

    fn is_full(&mut self) -> bool {
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

        let floor_string = (self.current_floor as u32).to_string();

        for (i, c) in floor_string.chars().enumerate() {
            ctx.set(9 + i, 8, RED, BLACK, to_cp437(c));
        }
    }
}

impl Floor {
    fn new(number: i32) -> Self {
        Floor {
            number: number,
            elevator: None,
            people: Vec::new(),
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
