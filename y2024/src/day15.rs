use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{error::Error, fmt::Display};

pub struct Solve {
    grid: Grid<char>,
    start: (usize, usize),
    instructions: Vec<Dir>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (maze, ins) = value.split_once("\n\n").ok_or("Not found")?;
        let grid = Grid::new_from(maze.to_string());
        let start = grid
            .iter()
            .find(|(_, _, c)| **c == '@')
            .map(|(y, x, _)| (y, x))
            .ok_or("@ not found")?;
        let instructions = ins
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Dir::N),
                '>' => Some(Dir::E),
                'v' => Some(Dir::S),
                '<' => Some(Dir::W),
                _ => None,
            })
            .collect();
        Ok(Solve {
            grid,
            start,
            instructions,
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut mgrid = self.grid.clone();
        let end = apply(self.start, &mut mgrid, &self.instructions);
        Ok(Box::new(
            end.iter()
                .filter(|(_, _, c)| **c == 'O')
                .map(|(y, x, _)| y * 100 + x)
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn apply<'a>(start: (usize, usize), grid: &'a mut Grid<char>, insts: &Vec<Dir>) -> &'a Grid<char> {
    let (mut fy, mut fx) = start;
    for dir in insts {
        let mvmt = grid
            .at_step(fy, fx, 1, dir)
            .map(|(t, c)| (t, *c))
            .and_then(|((ny, nx), c)| match c {
                '.' => Some((ny, nx)),
                'O' if prop_move(grid, dir, ny, nx) => Some((ny, nx)),
                _ => None,
            });
        if let Some((ny, nx)) = mvmt {
            grid.put(fy, fx, '.');
            fy = ny;
            fx = nx;
            grid.put(fy, fx, '@');
        }
    }
    grid
}

fn prop_move(grid: &mut Grid<char>, dir: &Dir, fy: usize, fx: usize) -> bool {
    // Need to scan ahead of the boxes, if free spot in dir then move all of them in dir.
    let mut moves = Vec::new();
    let has_space = 'sp: {
        let mut sy = fy;
        let mut sx = fx;
        while let Some(((ny, nx), c)) = grid.at_step(sy, sx, 1, dir) {
            match c {
                'O' => {
                    moves.push(((sy, sx), (ny, nx)));
                }
                '.' => {
                    moves.push(((sy, sx), (ny, nx)));
                    break 'sp true;
                }
                '#' => {
                    break;
                }
                x => unreachable!("Ran into char: {}", x),
            }
            sy = ny;
            sx = nx;
        }
        false
    };

    if has_space {
        // Apply mutations
        moves.iter().rev().for_each(|(p1, p2)| {
            let p1s = grid.at(p1.0, p1.1).unwrap();
            grid.put(p2.0, p2.1, *p1s);
        });
        true
    } else {
        false
    }
}
