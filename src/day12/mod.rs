use crate::utils;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DirectionChange {
    change: Change,
    value: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Change {
    North,
    West,
    South,
    East,
    Forward,
    Right,
    Left,
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Change::North => write!(f, "N"),
            Change::West => write!(f, "W"),
            Change::South => write!(f, "S"),
            Change::East => write!(f, "E"),
            Change::Forward => write!(f, "F"),
            Change::Right => write!(f, "R"),
            Change::Left => write!(f, "L"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "N"),
            Direction::West => write!(f, "W"),
            Direction::South => write!(f, "S"),
            Direction::East => write!(f, "E"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    east: i32,
    north: i32,
    facing: Direction,
}

fn part1(input: &Vec<DirectionChange>) {
    let now = SystemTime::now();

    let mut position = Position {
        east: 0,
        north: 0,
        facing: Direction::East,
    };

    for change in input {
        if change.change == Change::Forward {
            if position.facing == Direction::East {
                position.east += change.value as i32;
            }

            if position.facing == Direction::West {
                position.east -= change.value as i32;
            }

            if position.facing == Direction::North {
                position.north += change.value as i32;
            }

            if position.facing == Direction::South {
                position.north -= change.value as i32;
            }
        }

        if change.change == Change::Left {
            position.facing = rotate(position.facing, true, change.value);
        }

        if change.change == Change::Right {
            position.facing = rotate(position.facing, false, change.value);
        }

        if change.change == Change::North {
            position.north += change.value as i32;
        }

        if change.change == Change::South {
            position.north -= change.value as i32;
        }

        if change.change == Change::East {
            position.east += change.value as i32;
        }

        if change.change == Change::West {
            position.east -= change.value as i32;
        }
    }

    println!("Part 1: {}", distance(position.east, position.north));
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn rotate(currently_facing: Direction, rotate_left: bool, rotate_amount: u32) -> Direction {
    let mut current_degrees: i32 = match currently_facing {
        Direction::North => 0,
        Direction::East => 90,
        Direction::South => 180,
        Direction::West => 270,
    };

    if rotate_left {
        current_degrees -= rotate_amount as i32;
    } else {
        current_degrees += rotate_amount as i32;
    }

    while current_degrees < 0 {
        current_degrees = 360 + current_degrees;
    }

    while current_degrees > 360 {
        current_degrees -= 360;
    }
    return match current_degrees {
        0 => Direction::North,
        90 => Direction::East,
        180 => Direction::South,
        270 => Direction::West,
        360 => Direction::North,
        _ => panic!(
            "Unknown rotation: {} {} {} -> {}",
            currently_facing, rotate_left, rotate_amount, current_degrees
        ),
    };
}

fn distance(east: i32, north: i32) -> i32 {
    return east.abs() + north.abs();
}

fn part2(input: &Vec<DirectionChange>) {
    let now = SystemTime::now();

    let mut position = Position {
        east: 0,
        north: 0,
        facing: Direction::East,
    };

    let mut waypoint_position = Position {
        east: 10,
        north: 1,
        facing: Direction::East,
    };

    for change in input {
        if change.change == Change::Forward {
            position.north += (change.value as i32) * waypoint_position.north;
            position.east += (change.value as i32) * waypoint_position.east;
        }

        if change.change == Change::Left {
            waypoint_position = rotate_waypoint_around(waypoint_position, true, change.value);
        }

        if change.change == Change::Right {
            waypoint_position = rotate_waypoint_around(waypoint_position, false, change.value);
        }

        if change.change == Change::North {
            waypoint_position.north += change.value as i32;
        }

        if change.change == Change::South {
            waypoint_position.north -= change.value as i32;
        }

        if change.change == Change::East {
            waypoint_position.east += change.value as i32;
        }

        if change.change == Change::West {
            waypoint_position.east -= change.value as i32;
        }
    }

    println!("Part 2: {}", distance(position.east, position.north));
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn rotate_waypoint_around(
    waypoint_position: Position,
    rotate_left: bool,
    rotate_amount: u32,
) -> Position {
    if rotate_left {
        if rotate_amount == 90 {
            return Position {
                north: waypoint_position.east,
                east: -1 * waypoint_position.north,
                facing: waypoint_position.facing,
            };
        }

        if rotate_amount == 180 {
            return Position {
                north: -1 * waypoint_position.north,
                east: -1 * waypoint_position.east,
                facing: waypoint_position.facing,
            };
        }

        if rotate_amount == 270 {
            return Position {
                north: -1 * waypoint_position.east,
                east: waypoint_position.north,
                facing: waypoint_position.facing,
            };
        }
    }

    if rotate_amount == 90 {
        return rotate_waypoint_around(waypoint_position, true, 270);
    }

    if rotate_amount == 180 {
        return rotate_waypoint_around(waypoint_position, true, 180);
    }

    if rotate_amount == 270 {
        return rotate_waypoint_around(waypoint_position, true, 90);
    }

    panic!("Invalid rotation: {}", rotate_amount);
}

pub fn run() {
    println!("Running day12");

    let now = SystemTime::now();
    let input: String = utils::input::read("day12");
    let input_lines: Vec<&str> = input.split("\n").collect();

    let mut changes: Vec<DirectionChange> = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        let mut chars = line.chars();
        let direction_char = chars.next().unwrap();

        let direction = match direction_char {
            'N' => Change::North,
            'W' => Change::West,
            'S' => Change::South,
            'E' => Change::East,
            'F' => Change::Forward,
            'R' => Change::Right,
            'L' => Change::Left,
            _ => panic!("Unknown direction: {}", direction_char),
        };

        let value: u32 = chars.as_str().parse().unwrap();

        changes.push(DirectionChange {
            change: direction,
            value,
        });
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(&changes);
    part2(&changes);
}
