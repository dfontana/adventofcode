use itertools::Itertools;
use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{
    collections::{HashMap, VecDeque},
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
        let ans = find_shortcuts(&get_dists(&self.grid, &self.start));
        Ok(Box::new(format!("P1: {}, P2: {}", ans.0, ans.1)))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn get_dists(grid: &Grid<Tile>, start: &Loc) -> HashMap<Loc, usize> {
    let mut dists = HashMap::new();
    dists.insert(*start, 0);
    let mut frontier = VecDeque::from_iter(vec![(0, *start)]);
    while let Some((cost_to_here, (y, x))) = frontier.pop_front() {
        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(((ny, nx), t)) = grid.at_step(y, x, 1, &dir) {
                if *t == Tile::Wall {
                    continue;
                }
                if dists.contains_key(&(ny, nx)) {
                    continue;
                }
                dists.insert((ny, nx), cost_to_here + 1);
                frontier.push_back((cost_to_here + 1, (ny, nx)));
            }
        }
    }
    dists
}

fn find_shortcuts(dists: &HashMap<Loc, usize>) -> (usize, usize) {
    let mut short_2s = 0;
    let mut short_20s = 0;
    for (((y1, x1), c1), ((y2, x2), c2)) in dists
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .tuple_combinations()
    {
        let dist = (*x1 as isize - *x2 as isize).abs() + (*y1 as isize - *y2 as isize).abs();
        let cost = *c2 as isize - *c1 as isize - dist;
        if dist == 2 && cost >= 100 {
            short_2s += 1;
        }
        if dist < 21 && cost >= 100 {
            short_20s += 1;
        }
    }
    (short_2s, short_20s)
}
