use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{
    collections::{HashSet, VecDeque},
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
        let base_cost = find_path(&self.grid, &self.start);
        Ok(Box::new(
            find_cheaper_than(&self.grid, &self.start, base_cost - 2)
                .iter()
                .count(),
        ))
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
                frontier.push_back((
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
