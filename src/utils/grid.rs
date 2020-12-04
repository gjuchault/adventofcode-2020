use crate::utils;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{} y:{})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<TCell>
where
    TCell: Copy,
    TCell: fmt::Display,
{
    pub actual_position: Coordinates,
    pub width: u32,
    pub height: u32,
    grid: Vec<Vec<TCell>>,
}

impl<TCell> Grid<TCell>
where
    TCell: Copy,
    TCell: fmt::Display,
{
    pub fn at(&self, x: u32, y: u32) -> TCell {
        let resized_y = y as usize % self.grid.len();
        let resized_x = x as usize % self.grid[resized_y].len();

        return self.grid[resized_y][resized_x];
    }
}

impl<TCell> fmt::Display for Grid<TCell>
where
    TCell: Copy,
    TCell: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for column in &self.grid {
            for cell in column {
                write!(f, "{}", cell)?;
            }

            write!(f, "\n")?;
        }
        write!(f, "{}", self.actual_position)
    }
}

pub fn parse_grid<TCell, TIteratee>(
    input: String,
    default_value: TCell,
    iteratee: TIteratee,
    actual_position: Coordinates,
) -> Grid<TCell>
where
    TCell: Copy,
    TCell: fmt::Display,
    TIteratee: Fn(char) -> TCell,
{
    let height: u32 = utils::string::count_char(input.clone(), '\n');
    let width: u32 = (input.len() as u32 - height) / height;

    let mut grid: Vec<Vec<TCell>> =
        vec![vec![default_value; (width) as usize]; (height + 1) as usize];

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    for pos in 0..input.len() {
        let pos_uint = pos as u32;
        let char_at_pos = utils::string::char_at(input.clone(), pos_uint, '_');

        if char_at_pos == '\n' {
            x = 0;
            y += 1;
            continue;
        }

        grid[y as usize][x as usize] = iteratee(char_at_pos);

        x += 1;
    }

    return Grid {
        width,
        height,
        actual_position,
        grid,
    };
}
