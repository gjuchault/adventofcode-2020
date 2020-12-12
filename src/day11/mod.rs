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

type SeatPlan = utils::grid::Grid<Seat>;

fn part1(mut grid: SeatPlan) {
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
                if top_left == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if top_center == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if top_right == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if left == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if right == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if bottom_left == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if bottom_center == Seat::Occupied {
                    adjacent_occupied += 1;
                }
                if bottom_right == Seat::Occupied {
                    adjacent_occupied += 1;
                }

                if adjacent_occupied == 0 {
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

fn part2(mut grid: SeatPlan) {
    let now = SystemTime::now();

    // println!(
    //     "{}",
    //     grid.get_first_adjacent_cells_dir(9, 0, 1, 0, Seat::Floor)
    //         .unwrap_or(Seat::Floor)
    // );

    // return;

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
                ) = get_first_adjacent_seats(&grid, x, y);

                // start with all adjacent occupied and decrease
                let mut adjacents = 0;
                if top_left != Seat::Floor {
                    adjacents += 1;
                }
                if top_center != Seat::Floor {
                    adjacents += 1;
                }
                if top_right != Seat::Floor {
                    adjacents += 1;
                }
                if left != Seat::Floor {
                    adjacents += 1;
                }
                if right != Seat::Floor {
                    adjacents += 1;
                }
                if bottom_left != Seat::Floor {
                    adjacents += 1;
                }
                if bottom_center != Seat::Floor {
                    adjacents += 1;
                }
                if bottom_right != Seat::Floor {
                    adjacents += 1;
                }
                let mut adjacent_occupied = adjacents;

                if top_left == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if top_center == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if top_right == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if left == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if right == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if bottom_left == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if bottom_center == Seat::Empty {
                    adjacent_occupied -= 1;
                }
                if bottom_right == Seat::Empty {
                    adjacent_occupied -= 1;
                }

                if adjacent_occupied <= 0 {
                    loop_grid.set(x, y, Seat::Occupied);
                }
                if adjacent_occupied >= 5 {
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

fn get_first_adjacent_seats(
    grid: &SeatPlan,
    x: u32,
    y: u32,
) -> (Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat) {
    return (
        grid.get_first_adjacent_cells_dir(x, y, -1, -1, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, 0, -1, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, 1, -1, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, -1, 0, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, 1, 0, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, -1, 1, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, 0, 1, Seat::Floor)
            .unwrap_or(Seat::Floor),
        grid.get_first_adjacent_cells_dir(x, y, 1, 1, Seat::Floor)
            .unwrap_or(Seat::Floor),
    );
}

pub fn run() {
    println!("Running day11");

    let now = SystemTime::now();
    let input: String = utils::input::read("day11");

    let grid: SeatPlan = utils::grid::parse_grid(input.clone(), Seat::Empty, |x| match x {
        'L' => Seat::Empty,
        '#' => Seat::Occupied,
        '.' => Seat::Floor,
        _ => panic!("Unexpected character {}", x),
    });

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(grid.clone());
    part2(grid.clone());
}
