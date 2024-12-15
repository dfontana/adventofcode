use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
    maze: String,
    instructions: Vec<Dir>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (maze, ins) = value.split_once("\n\n").ok_or("Not found")?;
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
            maze: maze.to_string(),
            instructions,
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut mgrid = Grid::new_from(self.maze.clone());
        let start = mgrid
            .iter()
            .find(|(_, _, c)| **c == '@')
            .map(|(y, x, _)| (y, x))
            .ok_or("@ not found")?;
        let end = apply(start, &mut mgrid, &self.instructions);
        Ok(Box::new(
            end.iter()
                .filter(|(_, _, c)| **c == 'O')
                .map(|(y, x, _)| y * 100 + x)
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut mgrid = Grid::new_from(
            self.maze
                .chars()
                .map(|c| match c {
                    '#' => "##",
                    '.' => "..",
                    '@' => "@.",
                    'O' => "[]",
                    '\n' => "\n",
                    x => unreachable!("Unknown char hit: {:?}", x),
                })
                .collect(),
        );
        let start = mgrid
            .iter()
            .find(|(_, _, c)| **c == '@')
            .map(|(y, x, _)| (y, x))
            .ok_or("@ not found")?;
        let end = apply(start, &mut mgrid, &self.instructions);
        Ok(Box::new(
            end.iter()
                .filter(|(_, _, c)| **c == '[')
                .map(|(y, x, _)| y * 100 + x)
                .sum::<usize>(),
        ))
    }
}

fn apply<'a>(start: (usize, usize), grid: &'a mut Grid<char>, insts: &Vec<Dir>) -> &'a Grid<char> {
    let (mut fy, mut fx) = start;
    for dir in insts {
        let mvmt = prop_move(grid, dir, ((fy, fx), *grid.at(fy, fx).unwrap()));
        if let Some(moves) = mvmt {
            let mut touched = HashSet::new();
            moves.iter().for_each(|(_, to, set_c)| {
                touched.insert(to);
                grid.put(to.0, to.1, *set_c);
            });
            moves.iter().for_each(|(from, _, _)| {
                if touched.contains(from) {
                    return;
                }
                grid.put(from.0, from.1, '.');
            });

            let (_, (ny, nx), to_c) = *moves.last().unwrap();
            if to_c != '@' {
                panic!("Order lost");
            }
            fy = ny;
            fx = nx;
        }
    }
    grid
}

type Loc = (usize, usize);
type Move = (Loc, Loc, char);
fn prop_move(grid: &Grid<char>, dir: &Dir, mf: (Loc, char)) -> Option<Vec<Move>> {
    if *dir == Dir::E || *dir == Dir::W {
        rec_move_ew(grid, dir, mf)
    } else {
        rec_move_ns(grid, dir, mf)
    }
}
fn rec_move_ew(grid: &Grid<char>, dir: &Dir, (from, from_c): (Loc, char)) -> Option<Vec<Move>> {
    let (to_loc, to_c) = grid.at_step(from.0, from.1, 1, dir)?;
    if *to_c == '.' {
        return Some(vec![(from, to_loc, from_c)]);
    }
    if *to_c == '#' {
        return None;
    }
    let mut moves = rec_move_ew(grid, dir, (to_loc, *to_c))?;
    // If that can move, this can move
    moves.push((from, to_loc, from_c));
    Some(moves)
}

fn rec_move_ns(grid: &Grid<char>, dir: &Dir, (from, moving): (Loc, char)) -> Option<Vec<Move>> {
    match moving {
        '[' | ']' => {
            let dx: isize = if moving == '[' { 1 } else { -1 };
            let opp = if moving == '[' { ']' } else { '[' };
            let rx = from.1.checked_add_signed(dx)?;
            let (tl, lc) = grid.at_step(from.0, from.1, 1, dir)?;
            let (tr, rc) = grid.at_step(from.0, rx, 1, dir)?;
            if *lc == '#' || *rc == '#' {
                return None;
            }
            let mut moves = Vec::new();
            if *lc != '.' {
                moves.extend(rec_move_ns(grid, dir, (tl, *lc))?);
            }
            moves.push((from, tl, moving));
            moves.push(((from.0, rx), tr, opp));
            if *rc != '.' {
                moves.extend(rec_move_ns(grid, dir, (tr, *rc))?);
            }
            Some(moves)
        }
        'O' | '@' => {
            let (to_loc, to_c) = grid.at_step(from.0, from.1, 1, dir)?;
            if *to_c == '.' {
                return Some(vec![(from, to_loc, moving)]);
            }
            if *to_c == '#' {
                return None;
            }
            let mut moves = rec_move_ns(grid, dir, (to_loc, *to_c))?;
            moves.push((from, to_loc, moving));
            Some(moves)
        }
        '#' => None,
        _ => unreachable!("Moving something I shouldn't! {moving}"),
    }
}
