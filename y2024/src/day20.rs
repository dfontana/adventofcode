use itertools::Itertools;
use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{self, Display},
};

#[derive(PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Space,
    Wall,
}
impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => 'S',
                Tile::End => 'E',
                Tile::Space => '.',
                Tile::Wall => '#',
            }
        )
    }
}
type Loc = (usize, usize);
pub struct Solve {
    grid: Grid<Tile>,
    start: Loc,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let grid = Grid::new_from_map(value, |c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Space,
            'E' => Tile::End,
            'S' => Tile::Start,
            x => unreachable!("Invalid char hit: {x}"),
        });
        let start = grid
            .iter()
            .find_map(|(y, x, t)| match *t == Tile::Start {
                true => Some((y, x)),
                false => None,
            })
            .unwrap();
        Ok(Solve { grid, start })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(find_shortcuts(&get_dists(
            &self.grid,
            &self.start,
        ))))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn find_path(grid: &Grid<Tile>, start: &Loc) -> usize {
    let mut frontier = VecDeque::from_iter(vec![(0, *start)]);
    let mut seen = HashSet::new();
    while let Some((cost_to_here, (y, x))) = frontier.pop_front() {
        if grid.at(y, x).filter(|t| **t == Tile::End).is_some() {
            println!("target: {cost_to_here}");
            return cost_to_here;
        }
        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(((ny, nx), nt)) = grid.at_step(y, x, 1, &dir) {
                if *nt == Tile::Wall {
                    continue;
                }
                if seen.contains(&(ny, nx)) {
                    continue;
                }
                seen.insert((ny, nx));
                frontier.push_back((cost_to_here + 1, (ny, nx)));
            }
        }
    }
    unreachable!("Solution was guaranteed to exist")
}

fn find_cheaper_than(grid: &Grid<Tile>, start: &Loc, max_cost_allowed: usize) -> Vec<usize> {
    let path: HashSet<Loc> = HashSet::from_iter(vec![*start]);
    let mut frontier = VecDeque::from_iter(vec![(0, false, *start, path)]);
    let mut solution_costs = Vec::new();
    while let Some((cost_to_here, has_cheated, (y, x), in_path)) = frontier.pop_front() {
        if cost_to_here > max_cost_allowed {
            continue;
        }
        if grid.at(y, x).filter(|t| **t == Tile::End).is_some() {
            solution_costs.push(cost_to_here);
            continue;
        }

        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(((ny, nx), nt)) = grid.at_step(y, x, 1, &dir) {
                if *nt == Tile::Wall && has_cheated {
                    continue;
                }
                if in_path.contains(&(ny, nx)) {
                    continue;
                }
                let mut new_path = in_path.clone();
                new_path.insert((ny, nx));
                frontier.push_front((
                    cost_to_here + 1,
                    has_cheated || *nt == Tile::Wall,
                    (ny, nx),
                    new_path,
                ));
            }
        }
    }
    solution_costs
}

// 8944 is too high
fn get_dists(grid: &Grid<Tile>, start: &Loc) -> HashMap<Loc, usize> {
    let mut dists = HashMap::new();
    let mut frontier = VecDeque::from_iter(vec![(0, *start)]);
    let mut seen = HashSet::new();
    while let Some((cost_to_here, (y, x))) = frontier.pop_front() {
        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(((ny, nx), t)) = grid.at_step(y, x, 1, &dir) {
                if *t == Tile::Wall {
                    continue;
                }
                if seen.contains(&(ny, nx)) {
                    continue;
                }
                seen.insert((ny, nx));
                dists.insert((ny, nx), cost_to_here + 1);
                frontier.push_back((cost_to_here + 1, (ny, nx)));
            }
        }
    }
    dists
}

fn find_shortcuts(dists: &HashMap<Loc, usize>) -> usize {
    let mut shortcuts = 0;
    for (((y1, x1), c1), ((y2, x2), c2)) in dists.iter().tuple_combinations() {
        let dist = (*x1 as isize - *x2 as isize).abs() + (*y1 as isize - *y2 as isize).abs();
        if dist == 2 && c2 - c1 - dist as usize >= 100 {
            shortcuts += 1;
        }
    }
    shortcuts
}
