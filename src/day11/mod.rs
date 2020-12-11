use crate::utils;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
            Seat::Floor => write!(f, "."),
        }
    }
}

impl Default for Seat {
    fn default() -> Self {
        return Seat::Floor;
    }
}

type TreeMap = utils::grid::Grid<Seat>;

fn part1(mut grid: TreeMap) {
    let now = SystemTime::now();

    loop {
        let mut loop_grid = grid.clone();

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.at(x, y) == Seat::Floor {
                    continue;
                }
                let (
                    top_left,
                    top_center,
                    top_right,
                    left,
                    right,
                    bottom_left,
                    bottom_center,
                    bottom_right,
                ) = grid.get_adjacent_cells(x, y, Seat::Floor);
                let mut adjacent_occupied = 0;
                let mut adjacent_free = 0;
                if top_left == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if top_center == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if top_right == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if left == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if right == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if bottom_left == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if bottom_center == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }
                if bottom_right == Seat::Occupied {
                    adjacent_occupied += 1;
                } else {
                    adjacent_free += 1;
                }

                if adjacent_free == 8 {
                    loop_grid.set(x, y, Seat::Occupied);
                }
                if adjacent_occupied >= 4 {
                    loop_grid.set(x, y, Seat::Empty);
                }
            }
        }

        if loop_grid == grid {
            break;
        }

        grid = loop_grid.clone();
    }

    let mut result = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at(x, y) == Seat::Occupied {
                result += 1;
            }
        }
    }

    println!("Part 1: {}", result);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: &TreeMap) {
    let now = SystemTime::now();

    println!("Part 2: {}", 0);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day11");

    let now = SystemTime::now();
    let input: String = utils::input::read("day11");

    let grid: TreeMap = utils::grid::parse_grid(input.clone(), Seat::Empty, |x| match x {
        'L' => Seat::Empty,
        '#' => Seat::Occupied,
        '.' => Seat::Floor,
        _ => panic!("Unexpected character {}", x),
    });

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(grid.clone());
    part2(&grid);
}
