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
    // 1294459
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
    // grid.print();
    for dir in insts {
        let mvmt = grid
            .at_step(fy, fx, 1, dir)
            .map(|(t, c)| (t, *c))
            .and_then(|((ny, nx), c)| match c {
                '.' => Some((ny, nx)),
                '[' if (*dir == Dir::S || *dir == Dir::N) && prop_move(grid, dir, ny, nx, true) => {
                    // Clear the tile to the right of the bot
                    grid.put(ny, fx + 1, '.');
                    Some((ny, nx))
                }
                ']' if (*dir == Dir::S || *dir == Dir::N) && prop_move(grid, dir, ny, nx, true) => {
                    // Clear the tile to the left of the bot
                    grid.put(ny, fx - 1, '.');
                    Some((ny, nx))
                }
                '[' | ']' if prop_move(grid, dir, ny, nx, true) => Some((ny, nx)),
                'O' if prop_move(grid, dir, ny, nx, false) => Some((ny, nx)),
                _ => None,
            });
        if let Some((ny, nx)) = mvmt {
            grid.put(fy, fx, '.');
            fy = ny;
            fx = nx;
            grid.put(fy, fx, '@');
        }
        // grid.print();
    }
    // grid.print();
    grid
}

fn prop_move(grid: &mut Grid<char>, dir: &Dir, fy: usize, fx: usize, double_wide: bool) -> bool {
    // Need to scan ahead of the boxes, if free spot in dir then move all of them in dir.
    let mut moves = Vec::new();
    let has_space = 'sp: {
        let mut sy = fy;
        let mut sx = fx;
        let mut also_check_columns: HashSet<usize> = HashSet::from_iter(vec![sx]);
        let mut all_free_check: HashSet<usize> = HashSet::new();
        while let Some(((ny, nx), _)) = grid.at_step(sy, sx, 1, dir) {
            let mut n_cols: HashSet<usize> = HashSet::new();
            let mut hit_free = false;
            for ax in also_check_columns.iter() {
                let Some(bot_c) = grid.at(sy, *ax) else {
                    break 'sp false;
                };
                // println!("Checking: ({sy},{ax}) -> ({ny},{ax}) {bot_c}");
                match bot_c {
                    '#' => {
                        break 'sp false;
                    }
                    '.' if (!double_wide || *dir == Dir::W || *dir == Dir::E) => {
                        // moves.push(((sy, sx), (ny, nx)));
                        break 'sp true;
                    }
                    'O' => {
                        moves.push(((sy, sx), (ny, nx)));
                    }
                    // Double wide mode has additional behaviors
                    '[' | ']' if *dir == Dir::W || *dir == Dir::E => {
                        // This acts like single wide b/c it's not double height.
                        moves.push(((sy, sx), (ny, nx)));
                    }
                    // But if we hit a box edge N/S, we need to start looking for it's other edge
                    // This can create a cascade to look at
                    ']' if *dir == Dir::S || *dir == Dir::N => {
                        moves.push(((sy, *ax), (ny, *ax)));
                        if !n_cols.contains(&(ax - 1)) {
                            n_cols.insert(ax - 1);
                            moves.push(((sy, ax - 1), (ny, ax - 1)));
                        }
                    }
                    '[' if *dir == Dir::S || *dir == Dir::N => {
                        moves.push(((sy, *ax), (ny, *ax)));
                        if !n_cols.contains(&(ax + 1)) {
                            n_cols.insert(ax + 1);
                            moves.push(((sy, ax + 1), (ny, ax + 1)));
                        }
                    }
                    '.' if double_wide && (*dir == Dir::S || *dir == Dir::N) => {
                        // All items need ot be free to return true, otherwise it's false
                        all_free_check.insert(*ax);
                        moves.push(((sy, *ax), (ny, *ax)));
                        hit_free = true;
                    }
                    x => unreachable!("Ran into char: {}", x),
                }
                // println!("\t moves after: {moves:?}");
            }
            if hit_free {
                break 'sp all_free_check == also_check_columns;
            }
            also_check_columns.extend(n_cols);
            // println!("{:?}", also_check_columns);
            if *dir == Dir::W || *dir == Dir::E {
                also_check_columns.remove(&sx);
                also_check_columns.insert(nx);
            }
            sy = ny;
            sx = nx;
        }
        false
    };

    if has_space {
        // Apply mutations
        moves.iter().rev().for_each(|(p1, p2)| {
            let p1s = match grid.at(p1.0, p1.1) {
                Some(x) => x,
                None => {
                    // grid.print();
                    panic!(
                    "Tried to move out of the grid going {:?} from {:?} -> {:?} (originally {:?})",
                    dir, p1, p2, (fy, fx)
                );
                }
            };
            grid.put(p2.0, p2.1, *p1s);
        });
        true
    } else {
        false
    }
}
