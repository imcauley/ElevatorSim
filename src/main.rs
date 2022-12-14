#![warn(clippy::all, clippy::pedantic)]

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use rand::Rng;
use std::io::{stdout, Write};
use std::{thread, time};

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Still,
}

type People = Vec<Person>;
type Path = (i32, i32);

#[derive(Clone, Debug)]
struct Person {
    origin: i32,
    destination: i32,
}

struct Elevator {
    people: People,
    floor: i32,
    destination: i32,
    max_capacity: i32,
}

#[derive(Clone)]
struct Floor {
    number: i32,
    people: People,
}

impl Person {
    fn new_random_person(coming_in: bool, max_floor: i32) -> Self {
        let mut rng = rand::thread_rng();

        let origin = rng.gen_range(0..max_floor);
        let mut destination = 1;

        // majority of people will go to first floor
        if rng.gen_range(0..100) < 5 {
            destination = rng.gen_range(0..max_floor);
        }

        if coming_in {
            Person {
                origin: origin,
                destination: destination,
            }
        } else {
            Person {
                origin: destination,
                destination: origin,
            }
        }
    }

    fn print(&self) {
        println!(
            "Person | Origin {} | Destination {}",
            self.origin, self.destination
        );
    }

    fn going_in_direction(&self) -> Direction {
        if self.origin < self.destination {
            return Direction::Up;
        } else {
            return Direction::Down;
        }
    }
}

impl Floor {
    fn new(number: i32) -> Self {
        Floor {
            number: number,
            people: People::new(),
        }
    }

    fn print(&self) {
        print!("Floor {} | People {}", self.number, self.people.len());
    }
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            people: People::new(),
            floor: 1,
            max_capacity: 10,
            destination: 1,
        }
    }

    fn print(&self) {
        print!(
            "Evelator | {} People | Floor {} | Destination {}",
            self.people.len(),
            self.floor,
            self.destination
        )
    }

    fn tick(&mut self) {
        match self.going_in_direction() {
            Direction::Still => {}
            Direction::Down => self.floor -= 1,
            Direction::Up => self.floor += 1,
        }
    }

    fn set_destionation(&mut self, destination: i32) {
        match self.going_in_direction() {
            Direction::Still => self.destination = destination,
            _ => {}
        }
    }

    fn add_person_to_elevator(&mut self, person: Person) {
        if person.destination > self.destination {
            self.set_destionation(person.destination);
        }

        self.people.push(person);
    }

    fn going_in_direction(&self) -> Direction {
        if self.floor < self.destination {
            return Direction::Up;
        } else if self.floor > self.destination {
            return Direction::Down;
        } else {
            return Direction::Still;
        }
    }

    fn has_capacity(&self) -> bool {
        (self.people.len() as i32) < self.max_capacity
    }
}

fn same_direction(elevator: &Elevator, person: &Person) -> bool {
    elevator.going_in_direction() == person.going_in_direction()
}

fn path_direction(path: Path) -> Direction {
    if path.0 < path.1 {
        return Direction::Up;
    } else {
        return Direction::Down;
    }
}

fn call_elevators(elevators: &mut Vec<Elevator>, paths: Vec<Path>) {
    for path in paths {
        for elevator in elevators.iter_mut() {
            match elevator.going_in_direction() {
                Direction::Still => {
                    elevator.set_destionation(path.0);
                    break;
                }
                Direction::Up => {
                    if path.0 >= elevator.floor {
                        elevator.set_destionation(path.0);
                        break;
                    }
                }
                Direction::Down => {
                    if path.0 <= elevator.floor {
                        elevator.set_destionation(path.0);
                        break;
                    }
                }
            }
        }
    }
}

fn get_people_waiting(floors: Vec<Floor>) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    for floor in floors {
        for person in floor.people {
            paths.push((floor.number, person.destination));
        }
    }

    paths
}

fn transfer_floor_to_elevator(floor: &mut Floor, elevator: &mut Elevator) {
    let mut remaining_people: People = Vec::new();

    for person in &mut floor.people {
        if elevator.has_capacity() {
            elevator.add_person_to_elevator(person.clone());
        } else {
            remaining_people.push(person.clone());
        }
    }

    floor.people = remaining_people;
}

fn transfer_elevator_to_floor(floor: &mut Floor, elevator: &mut Elevator) {
    let mut remaining_people = People::new();

    for person in &mut elevator.people {
        if floor.number == person.destination {
            floor.people.push(person.clone());
        } else {
            remaining_people.push(person.clone());
        }
    }

    elevator.people = remaining_people;
}

fn similation_tick(elevators: &mut Vec<Elevator>, floors: &mut Vec<Floor>) {
    // generate new people
    for _ in 0..2 {
        let current_person = Person::new_random_person(true, 10);
        floors[current_person.origin as usize]
            .people
            .push(current_person);
    }

    // change elevator directions
    // TODO: this only needs origin floors
    let paths = get_people_waiting(floors.clone());
    call_elevators(elevators, paths);
    for elevator in elevators.iter_mut() {
        elevator.tick();
    }

    // transfer people from elevators to floors
    for floor in floors.iter_mut() {
        for elevator in elevators.iter_mut() {
            if elevator.floor == floor.number {
                transfer_elevator_to_floor(floor, elevator);
            }
        }
    }

    // transfer people from floors to elevators
    for floor in floors.iter_mut() {
        for elevator in elevators.iter_mut() {
            if elevator.floor == floor.number {
                transfer_floor_to_elevator(floor, elevator);
            }
        }
    }
}

fn print_simulation(elevators: &mut Vec<Elevator>, floors: &mut Vec<Floor>) -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // output floors
    for (index, floor) in floors.iter().enumerate() {
        stdout.queue(cursor::MoveTo(3, (index + 3) as u16))?;
        floor.print();
    }

    // output elevators
    for (index, elevator) in elevators.iter().enumerate() {
        stdout.queue(cursor::MoveTo(25, (index + 3) as u16))?;
        elevator.print();
    }

    stdout.flush()?;
    Ok(())
}

fn main() {
    // setup
    let mut floors = Vec::new();
    for index in 0..10 {
        floors.push(Floor::new(index));
    }

    let mut elevators = Vec::new();
    for _ in 0..3 {
        elevators.push(Elevator::new());
    }

    for _ in 0..50 {
        similation_tick(&mut elevators, &mut floors);
        print_simulation(&mut elevators, &mut floors).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
