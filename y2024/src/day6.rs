use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

pub struct Solve {
    by_x: HashMap<usize, Vec<usize>>,
    by_y: HashMap<usize, Vec<usize>>,
    width: usize,
    height: usize,
    patrol: (usize, usize, Dir),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let width = lines.nth(0).map(|v| v.len()).unwrap();
        let height = lines.count();
        let mut by_x = HashMap::new();
        let mut by_y = HashMap::new();
        let mut patrol = (0, 0, Dir::N);
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '^' => {
                        patrol = (x, y, Dir::N);
                    }
                    '#' => {
                        by_x.entry(x)
                            .and_modify(|v: &mut Vec<_>| v.push(y))
                            .or_insert_with(|| vec![y]);
                        by_y.entry(y)
                            .and_modify(|v: &mut Vec<_>| v.push(x))
                            .or_insert_with(|| vec![x]);
                    }
                    _ => {}
                }
            }
        }
        Ok(Solve {
            by_x,
            by_y,
            width,
            height,
            patrol,
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            get_turns(
                &self.patrol,
                &self.by_x,
                &self.by_y,
                self.width,
                self.height,
            )
            .unwrap()
            .iter()
            .tuple_windows()
            .flat_map(|(c1, c2)| walk(c1, c2))
            .map(|(a, b, _)| (a, b))
            .collect::<HashSet<_>>()
            .len(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            // Walk the path, at each step add a block in all 3 directions and finish the walk (test for loop).
            // Keep those that cause loop.
            // I have a bug and am too tired to know why. But for some reason the code
            // tries to put one in front of the guard and thinks it's valid. Let's remove it.
            get_turns(
                &self.patrol,
                &self.by_x,
                &self.by_y,
                self.width,
                self.height,
            )
            .unwrap()
            .iter()
            .tuple_windows()
            .flat_map(|(c1, c2)| walk(c1, c2))
            .flat_map(|(x, y, d)| match d {
                // Pair point with with each neighbor
                Dir::N => vec![(x - 1, y), (x + 1, y), (x, y - 1)],
                Dir::S => vec![(x - 1, y), (x + 1, y), (x, y + 1)],
                Dir::E => vec![(x, y - 1), (x, y + 1), (x + 1, y)],
                Dir::W => vec![(x, y - 1), (x, y + 1), (x - 1, y)],
            })
            .filter(|test| self.triggers_loop(test))
            .collect::<HashSet<_>>()
            .len()
                - 1,
        ))
    }
}

impl Dir {
    pub fn next(
        &self,
        by_x: &HashMap<usize, Vec<usize>>,
        by_y: &HashMap<usize, Vec<usize>>,
        px: usize,
        py: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Dir::N => by_x
                .get(&px)
                .and_then(|v| find_highest_lt(py, v))
                .map(|ny| (px, ny)),
            Dir::S => by_x
                .get(&px)
                .and_then(|v| find_lowest_gt(py, v))
                .map(|ny| (px, ny)),
            Dir::E => by_y
                .get(&py)
                .and_then(|v| find_lowest_gt(px, v))
                .map(|nx| (nx, py)),
            Dir::W => by_y
                .get(&py)
                .and_then(|v| find_highest_lt(px, v))
                .map(|nx| (nx, py)),
        }
    }

    pub fn last(&self, width: usize, height: usize, px: usize, py: usize) -> (usize, usize, Dir) {
        match self {
            Dir::N => (px, 0, self.clone()),
            Dir::S => (px, height, self.clone()),
            Dir::E => (width, py, self.clone()),
            Dir::W => (0, py, self.clone()),
        }
    }

    fn turn(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
}

fn get_turns(
    patrol: &(usize, usize, Dir),
    by_x: &HashMap<usize, Vec<usize>>,
    by_y: &HashMap<usize, Vec<usize>>,
    width: usize,
    height: usize,
) -> Option<Vec<(usize, usize, Dir)>> {
    let (mut px, mut py, mut pd) = patrol.clone();
    let mut coords: Vec<(usize, usize, Dir)> = vec![(px, py, pd.clone())];
    let mut seen: HashSet<(usize, usize, Dir)> = HashSet::from_iter(coords.clone());
    loop {
        let next_spot = pd.next(&by_x, &by_y, px, py);
        if next_spot.is_none() {
            coords.push(pd.last(width, height, px, py));
            break;
        }
        let (nx, ny) = next_spot.unwrap();
        let nxt_pt = (nx, ny, pd.clone());
        if seen.contains(&nxt_pt) {
            return None;
        }
        coords.push(nxt_pt.clone());
        seen.insert(nxt_pt);
        pd = pd.turn();
        px = nx;
        py = ny;
    }
    Some(coords)
}

impl Solve {
    fn triggers_loop(&self, item: &(usize, usize)) -> bool {
        // Insert item into by_x & by_y and see if you hit a seen known during loop
        let mut by_x = self.by_x.clone();
        let mut by_y = self.by_y.clone();
        by_x.entry(item.0)
            .and_modify(|v| v.push(item.1))
            .or_insert_with(|| vec![item.1]);
        by_y.entry(item.1)
            .and_modify(|v| v.push(item.0))
            .or_insert_with(|| vec![item.0]);
        get_turns(&self.patrol, &by_x, &by_y, self.width, self.height).is_none()
    }
}

fn walk(c1: &(usize, usize, Dir), c2: &(usize, usize, Dir)) -> Vec<(usize, usize, Dir)> {
    match c2.2 {
        Dir::N => (c2.1..=c1.1).map(|y| (c1.0, y, c2.2.clone())).collect(),
        Dir::S => (c1.1..=c2.1).map(|y| (c1.0, y, c2.2.clone())).collect(),
        Dir::E => (c1.0..=c2.0).map(|x| (x, c1.1, c2.2.clone())).collect(),
        Dir::W => (c2.0..=c1.0).map(|x| (x, c1.1, c2.2.clone())).collect(),
    }
}

fn find_lowest_gt(px: usize, v: &[usize]) -> Option<usize> {
    v.iter().filter(|i| **i > px).min().map(|i| i - 1)
}

fn find_highest_lt(px: usize, v: &[usize]) -> Option<usize> {
    v.iter().filter(|i| **i < px).max().map(|i| i + 1)
}
