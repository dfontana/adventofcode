use rust_util::grid::{Dir, Grid};
use rust_util::Day;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
use std::{error::Error, fmt::Display};

type Loc = (usize, usize);
pub struct Solve {
    grid: Grid<char>,
    start: Loc,
    end: Loc,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let grid = Grid::new_from(value);
        let start_pos = grid
            .iter()
            .find_map(|(y, x, c)| match c {
                'S' => Some((y, x)),
                _ => None,
            })
            .unwrap();
        let end_pos = grid
            .iter()
            .find_map(|(y, x, c)| match c {
                'E' => Some((y, x)),
                _ => None,
            })
            .unwrap();
        Ok(Solve {
            grid,
            start: (start_pos.0, start_pos.1),
            end: (end_pos.0, end_pos.1),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(dijkstra(
            &self.grid,
            (self.start, Dir::E),
            &self.end,
        )))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

// Turns cost 1000
// Moves cost 1
fn dijkstra(grid: &Grid<char>, start: (Loc, Dir), goal: &Loc) -> usize {
    // cache of costs to get to a loc, so we can find cheaper paths
    let mut cost_to_loc = HashMap::new();
    // Current explorations: (cost-to-point,(y,x,dir))
    let mut q = BinaryHeap::from_iter([Reverse((0, (start.0 .0, start.0 .1, start.1)))]);

    while let Some(Reverse((cost, (y, x, d)))) = q.pop() {
        if (y, x) == *goal {
            return cost;
        }

        // Skip this path if we've found a cheaper path to here
        if cost_to_loc
            .get(&(y, x, d.clone()))
            .is_some_and(|&c| cost > c)
        {
            continue;
        }

        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            if dir == d.rev() {
                continue;
            }

            // Attempt to travel in dir, 1 step
            let Some(((ny, nx), tile)) = grid.at_step(y, x, 1, &dir) else {
                continue;
            };
            if *tile == '#' {
                // Can't travel into walls
                continue;
            }

            // Taking a step costs 1. But turning costs 1000, per 90deg.
            // Since we don't allow reverse directions, this means we turn at most 1
            let next_cost = cost + 1 + (1000 * (if d == dir { 0 } else { 1 }));
            let key = (ny, nx, dir.clone());
            if next_cost < *cost_to_loc.get(&key).unwrap_or(&usize::MAX) {
                cost_to_loc.insert(key.clone(), next_cost);
                q.push(Reverse((next_cost, key.clone())));
            }
        }
    }
    unreachable!("Did not find path to target")
}
