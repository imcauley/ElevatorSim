#![warn(clippy::all, clippy::pedantic)]

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Still,
}

type People = Vec<Person>;

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

struct Floor {
    number: i32,
    people: People,
}

impl Person {
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

    fn going_in_direction(&self) -> Direction {
        if self.floor < self.destination {
            return Direction::Up;
        } else if self.floor < self.destination {
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

fn transfer_floor_to_elevator(floor: &mut Floor, elevator: &mut Elevator) {
    let mut remaining_people: People = Vec::new();

    for person in &mut floor.people {
        if elevator.has_capacity() && same_direction(elevator, person) {
            elevator.people.push(person.clone());
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

fn main() {
    let mut floors = Vec::new();
    for index in 0..10 {
        floors.push(Floor::new(index));
    }

    let mut elevators = Vec::new();
    for _ in 0..3 {
        elevators.push(Elevator::new());
    }

    // change elevator directions
    // transfer people from elevators to floors
    for floor in &mut floors {
        for elevator in &mut elevators {
            if elevator.floor == floor.number {
                transfer_elevator_to_floor(floor, elevator);
            }
        }
    }

    // transfer people from floors to elevators
    for floor in &mut floors {
        for elevator in &mut elevators {
            if elevator.floor == floor.number {
                transfer_floor_to_elevator(floor, elevator);
            }
        }
    }
}
