use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

use crate::grid::{Dir, Grid};

pub fn dijkstra(grid: Grid<usize>, minstep: usize, maxstep: usize) -> usize {
  let mut dists = HashMap::new();
  let mut q = BinaryHeap::from_iter([Reverse((0, (0, 0, Dir::Idle)))]);

  while let Some(Reverse((cost, (y, x, d)))) = q.pop() {
    // Goal Checking
    if (y, x) == grid.bottom_right() {
      return cost;
    }

    // Skip this path if we've found a shorter path to here
    if dists.get(&(y, x, d.clone())).is_some_and(|&c| cost > c) {
      continue;
    }

    for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
      // Skip going in same direction or reverse direction
      if dir == d || dir == d.rev() {
        continue;
      }

      let mut next_cost = cost;
      for dist in 1..=maxstep {
        let Some(((rr, cc), cost)) = grid.at_step(y, x, dist, dir.clone()) else {
          continue;
        };
        next_cost += cost;
        if dist < minstep {
          continue;
        }

        let key = (rr, cc, dir.clone());
        if next_cost < *dists.get(&key).unwrap_or(&usize::MAX) {
          dists.insert(key.clone(), next_cost);
          q.push(Reverse((next_cost, key.clone())));
        }
      }
    }
  }
  unreachable!("Did not find path to target")
}
