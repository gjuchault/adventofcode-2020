use crate::utils;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
enum CellContent {
    Tree,
    Snow,
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellContent::Tree => write!(f, "#"),
            CellContent::Snow => write!(f, "."),
        }
    }
}

type TreeMap = utils::grid::Grid<CellContent>;

fn part1(grid: TreeMap) {
    let mut trees_encountered: u32 = 0;

    let mut next_position = grid.actual_position;

    while next_position.y <= grid.height {
        next_position = utils::grid::Coordinates {
            x: next_position.x + 3,
            y: next_position.y + 1,
        };

        if grid.at(next_position.x, next_position.y) == CellContent::Tree {
            trees_encountered += 1;
        }
    }

    println!("Part 1: {}", trees_encountered);
}

pub fn run() {
    println!("Running day3");
    let input: String = utils::input::read("day3");
    let grid: TreeMap = utils::grid::parse_grid(
        input.clone(),
        CellContent::Snow,
        |x| match x {
            '.' => CellContent::Snow,
            '#' => CellContent::Tree,
            _ => CellContent::Snow,
        },
        utils::grid::Coordinates { x: 0, y: 0 },
    );

    part1(grid);
}
