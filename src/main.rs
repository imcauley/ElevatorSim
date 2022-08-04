#![warn(clippy::all, clippy::pedantic)]

use std::mem::take;

type People = Vec<Person>;

#[derive(Clone, Debug)]
struct Person {
    origin: i32,
    destination: i32,
}

struct Elevator {
    people: People,
    floor: i32,
    max_capacity: i32,
}

struct Floor {
    number: i32,
    people: People,
}

impl Floor {
    fn new(number: i32) -> Self {
        Floor {
            number: number,
            people: People::new(),
        }
    }
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            people: People::new(),
            floor: 1,
            max_capacity: 10,
        }
    }

    fn has_capacity(&self) -> bool {
        true
    }
}

fn transfer_floor_to_elevator(floor: &mut Floor, elevator: &mut Elevator) {
    let mut remaining_people: People = Vec::new();

    for person in &mut floor.people {
        if elevator.has_capacity() {
            elevator.people.push(person.clone());
        } else {
            remaining_people.push(person.clone());
        }
    }

    floor.people = remaining_people;
}

fn main() {
    let mut floors = Vec::new();
    for index in 0..10 {
        floors.push(Floor::new(index));
    }

    let mut elevators = Vec::new();
    for _ in 0..3 {
        elevators.push(Elevator::new());
    }

    for floor in &mut floors {
        for elevator in &mut elevators {
            if elevator.floor == floor.number {
                transfer_floor_to_elevator(floor, elevator);
            }
        }
    }
}
