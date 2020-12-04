use crate::utils;
use std::fmt;
use std::time::SystemTime;

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

fn count_trees_slope(grid: TreeMap, delta_x: u32, delta_y: u32) -> u32 {
    let mut trees_encountered: u32 = 0;

    let mut next_position = utils::grid::Coordinates { x: 0, y: 0 };

    while next_position.y < grid.height {
        next_position = utils::grid::Coordinates {
            x: next_position.x + delta_x,
            y: next_position.y + delta_y,
        };

        if grid.at(next_position.x, next_position.y) == CellContent::Tree {
            trees_encountered += 1;
        }
    }

    return trees_encountered;
}

fn part1(grid: TreeMap) {
    let now = SystemTime::now();

    let trees_encountered = count_trees_slope(grid, 3, 1);

    println!("Part 1: {}", trees_encountered);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(grid: TreeMap) {
    let now = SystemTime::now();
    let deltas = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut score: u32 = 1;
    for (delta_x, delta_y) in deltas {
        let trees_encountered = count_trees_slope(grid.clone(), delta_x, delta_y);

        println!(
            "For delta ({}, {}): {} trees",
            delta_x, delta_y, trees_encountered
        );

        score *= trees_encountered;
    }

    println!("Part 2: {}", score);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day3");
    let now = SystemTime::now();
    let input: String = utils::input::read("day3");
    let grid: TreeMap = utils::grid::parse_grid(input.clone(), CellContent::Snow, |x| match x {
        '.' => CellContent::Snow,
        '#' => CellContent::Tree,
        _ => panic!("Unexpected character {}", x),
    });

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());
    part1(grid.clone());
    part2(grid.clone());
}
