use rust_util::{grid::Dir, Day};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

type Loc = (usize, usize);
pub struct Solve {
    bytes: Vec<Loc>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            bytes: value
                .lines()
                .filter_map(|l| l.split_once(","))
                .map(|(x, y)| (y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap()))
                .collect(),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let (gx, gy) = (70, 70);
        let (mx, my) = (gx + 1, gy + 1);
        let falling = 1024;
        let walls = self
            .bytes
            .iter()
            .take(falling)
            .fold(HashMap::new(), |mut acc, (y, x)| {
                acc.entry(y)
                    .and_modify(|v: &mut HashSet<usize>| {
                        v.insert(*x);
                    })
                    .or_insert_with(|| HashSet::from_iter(vec![*x]));
                acc
            });

        let mut frontier = VecDeque::from_iter(vec![(0, 0, 0)]);
        let mut seen: HashSet<Loc> = HashSet::from_iter(vec![(0, 0)]);
        while let Some((y, x, steps)) = frontier.pop_front() {
            if y == gy && x == gx {
                return Ok(Box::new(steps));
            }
            for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
                if let Some((ny, nx)) = step(y, x, 1, &my, &mx, &dir) {
                    if seen.contains(&(ny, nx)) {
                        continue;
                    }
                    if walls.get(&ny).filter(|v| v.contains(&nx)).is_some() {
                        // Can't walk here
                        continue;
                    }
                    seen.insert((ny, nx));
                    frontier.push_back((ny, nx, steps + 1));
                }
            }
        }
        Ok(Box::new(0))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

pub fn step(
    y: usize,
    x: usize,
    step: usize,
    my: &usize,
    mx: &usize,
    dir: &Dir,
) -> Option<(usize, usize)> {
    match dir {
        Dir::W => x.checked_sub(step).map(|x| (y, x)),
        Dir::N => y.checked_sub(step).map(|y| (y, x)),
        Dir::E => Some(x + step).filter(|x| *x < *mx).map(|x| (y, x)),
        Dir::S => Some(y + step).filter(|y| *y < *my).map(|y| (y, x)),
        Dir::Idle => Some((y, x)),
    }
}
