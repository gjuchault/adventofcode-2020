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
    TCell: PartialEq,
    TCell: fmt::Display,
{
    pub width: u32,
    pub height: u32,
    grid: Vec<Vec<TCell>>,
}

impl<TCell> PartialEq for Grid<TCell>
where
    TCell: Copy,
    TCell: PartialEq,
    TCell: fmt::Display,
{
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width {
            return false;
        }

        if self.height != other.height {
            return false;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.at(x, y) != other.at(x, y) {
                    return false;
                }
            }
        }

        return true;
    }
}

impl<TCell> Grid<TCell>
where
    TCell: Copy,
    TCell: PartialEq,
    TCell: fmt::Display,
{
    pub fn at(&self, x: u32, y: u32) -> TCell {
        let resized_y = y as usize % self.grid.len();
        let resized_x = x as usize % self.grid[resized_y].len();

        return self.grid[resized_y][resized_x];
    }

    pub fn set(&mut self, x: u32, y: u32, value: TCell) {
        self.grid[y as usize][x as usize] = value;
    }

    pub fn get_adjacent_cells(
        &self,
        x: u32,
        y: u32,
        default: TCell,
    ) -> (TCell, TCell, TCell, TCell, TCell, TCell, TCell, TCell) {
        let top_left = if y >= 1 && x >= 1 {
            self.at(x - 1, y - 1)
        } else {
            default
        };
        let top_center = if y >= 1 { self.at(x, y - 1) } else { default };
        let top_right = if y >= 1 && x + 1 < self.width {
            self.at(x + 1, y - 1)
        } else {
            default
        };
        let left = if x >= 1 { self.at(x - 1, y) } else { default };
        let right = if x + 1 < self.width {
            self.at(x + 1, y)
        } else {
            default
        };
        let bottom_left = if x >= 1 && y + 1 < self.height {
            self.at(x - 1, y + 1)
        } else {
            default
        };
        let bottom_center = if y + 1 < self.height {
            self.at(x, y + 1)
        } else {
            default
        };
        let bottom_right = if x + 1 < self.width && y + 1 < self.height {
            self.at(x + 1, y + 1)
        } else {
            default
        };

        return (
            top_left,
            top_center,
            top_right,
            left,
            right,
            bottom_left,
            bottom_center,
            bottom_right,
        );
    }

    pub fn get_first_adjacent_cells_dir(
        &self,
        x: u32,
        y: u32,
        delta_x: i32,
        delta_y: i32,
        ignore_cell: TCell,
    ) -> Option<TCell> {
        let mut current_x = x;
        let mut current_y = y;
        let mut current_cell: TCell;
        // println!("{}", self.at(x, y));
        loop {
            // out of bound -> default
            if (current_x as i32 + delta_x < 0)
                || (current_y as i32 + delta_y < 0)
                || (current_x as i32 + delta_x > (self.width - 1) as i32)
                || (current_y as i32 + delta_y > (self.height - 1) as i32)
            {
                return None;
            }

            let next_x = current_x as i32 + delta_x;
            let next_y = current_y as i32 + delta_y;

            current_cell = self.at(next_x as u32, next_y as u32);
            // println!("{}x{}: {}", next_x, next_y, current_cell);
            if current_cell != ignore_cell {
                return Some(current_cell);
            }
            current_x = next_x as u32;
            current_y = next_y as u32;
        }
    }
}

impl<TCell> fmt::Display for Grid<TCell>
where
    TCell: Copy,
    TCell: PartialEq,
    TCell: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for column in &self.grid {
            for cell in column {
                write!(f, "{}", cell)?;
            }

            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

pub fn parse_grid<TCell, TIteratee>(
    input: String,
    default_value: TCell,
    iteratee: TIteratee,
) -> Grid<TCell>
where
    TCell: Copy,
    TCell: PartialEq,
    TCell: fmt::Display,
    TIteratee: Fn(char) -> TCell,
{
    let height: u32 = utils::string::count_char(input.clone(), '\n') + 1;
    let width: u32 = (input.len() as u32 - height) / (height) + 1;
    let mut grid: Vec<Vec<TCell>> = vec![vec![default_value; (width) as usize]; (height) as usize];

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
        grid,
    };
}
