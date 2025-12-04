use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{error::Error, fmt::Display};

pub struct Solve {
    grid: Grid<Tile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Roll,
    Empty,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '@' => Tile::Roll,
            '.' => Tile::Empty,
            x => unreachable!("Unknown character hit: {x}"),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Roll => '@',
                Tile::Empty => '.',
            }
        )
    }
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            grid: Grid::new_from(value),
        })
    }
}

fn removable(grid: &Grid<Tile>) -> Vec<(usize, usize)> {
    grid.iter()
        .filter(|(y, x, t)| {
            if **t == Tile::Empty {
                return false;
            }
            let rolls_around = [
                Dir::N,
                Dir::E,
                Dir::S,
                Dir::W,
                Dir::NE,
                Dir::NW,
                Dir::SE,
                Dir::SW,
            ]
            .iter()
            .filter_map(|d| grid.at_step(*y, *x, 1, d))
            .filter(|(_, t)| **t == Tile::Roll)
            .count();
            rolls_around < 4
        })
        .map(|(y, x, _)| (y, x))
        .collect()
}

fn remove(grid: &mut Grid<Tile>, pos: &Vec<(usize, usize)>) {
    for (y, x) in pos {
        grid.put(*y, *x, Tile::Empty);
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(removable(&self.grid).len()))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut grid = self.grid.clone();
        let mut removed = 0;
        loop {
            let rm = removable(&grid);
            if rm.is_empty() {
                break;
            }
            removed += rm.len();
            remove(&mut grid, &rm);
        }
        Ok(Box::new(removed))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            13
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            43
        );
    }
}
