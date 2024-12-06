use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub struct Solve {
    by_x: HashMap<usize, Vec<usize>>,
    by_y: HashMap<usize, Vec<usize>>,
    width: usize,
    height: usize,
    patrol: (usize, usize, Dir),
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
                    '>' => {
                        patrol = (x, y, Dir::E);
                    }
                    'V' => {
                        patrol = (x, y, Dir::S);
                    }
                    '<' => {
                        patrol = (x, y, Dir::W);
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
    // 229 -> Too low
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        let (mut px, mut py, mut pd) = self.patrol.clone();
        let mut coords: Vec<(usize, usize, Dir)> = vec![(px, py, pd.clone())];
        loop {
            let next_spot = match pd {
                Dir::N => self
                    .by_x
                    .get(&px)
                    .and_then(|v| find_highest_lt(py, v))
                    .map(|ny| (px, ny)),
                Dir::S => self
                    .by_x
                    .get(&px)
                    .and_then(|v| find_lowest_gt(py, v))
                    .map(|ny| (px, ny)),
                Dir::E => self
                    .by_y
                    .get(&py)
                    .and_then(|v| find_lowest_gt(px, v))
                    .map(|nx| (nx, py)),
                Dir::W => self
                    .by_y
                    .get(&py)
                    .and_then(|v| find_highest_lt(px, v))
                    .map(|nx| (nx, py)),
            };
            if next_spot.is_none() {
                // Off the board, clamp to bounds and break
                coords.push(match pd {
                    Dir::N => (px, 0, pd),
                    Dir::S => (px, self.height, pd),
                    Dir::E => (self.width, py, pd),
                    Dir::W => (0, py, pd),
                });
                break;
            }
            let (nx, ny) = next_spot.unwrap();
            coords.push((nx, ny, pd.clone()));
            // Otherwise turn right from the new spot
            pd = match pd {
                Dir::N => Dir::E,
                Dir::S => Dir::W,
                Dir::E => Dir::S,
                Dir::W => Dir::N,
            };
            px = nx;
            py = ny;
        }
        // debug(&coords, self);
        Ok(Box::new(count_distinct_path(&coords)))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(Box::new(1))
    }
}

fn debug(coords: &[(usize, usize, Dir)], solve: &Solve) {
    let path = coords
        .iter()
        .tuple_windows()
        .flat_map(|(c1, c2)| walk(c1, c2))
        .collect::<HashSet<_>>();
    for (y, v) in vec![vec!['.'; solve.width]; solve.height + 1]
        .iter()
        .enumerate()
    {
        for (x, i) in v.iter().enumerate() {
            if solve.patrol.0 == x && solve.patrol.1 == y {
                print!("^");
            } else if solve.by_y.get(&y).filter(|v| v.contains(&x)).is_some() {
                print!("#");
            } else if path.contains(&(x, y)) {
                print!("X");
            } else {
                print!("{}", i);
            }
        }
        println!();
    }
}

fn count_distinct_path(coords: &[(usize, usize, Dir)]) -> usize {
    // Walk the list of coordinates and count each unique (x,y) visited
    coords
        .iter()
        .tuple_windows()
        .flat_map(|(c1, c2)| walk(c1, c2))
        .collect::<HashSet<_>>()
        .len()
}

fn walk(c1: &(usize, usize, Dir), c2: &(usize, usize, Dir)) -> Vec<(usize, usize)> {
    match c2.2 {
        Dir::N => (c2.1..=c1.1).map(|y| (c1.0, y)).collect(),
        Dir::S => (c1.1..=c2.1).map(|y| (c1.0, y)).collect(),
        Dir::E => (c1.0..=c2.0).map(|x| (x, c1.1)).collect(),
        Dir::W => (c2.0..=c1.0).map(|x| (x, c1.1)).collect(),
    }
}

fn find_lowest_gt(px: usize, v: &[usize]) -> Option<usize> {
    v.iter().filter(|i| **i > px).min().map(|i| i - 1)
}

fn find_highest_lt(px: usize, v: &[usize]) -> Option<usize> {
    v.iter().filter(|i| **i < px).max().map(|i| i + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();

        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            solve
                .p1()
                .map(|v| format!("{}", v).parse::<usize>().unwrap())
                .unwrap(),
            41
        );
    }
}
