use rust_util::Day;
use std::{collections::HashMap, error::Error};

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
        let height = lines.count() + 1;
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
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        let (mut px, mut py, mut pd) = self.patrol.clone();
        let mut coords: Vec<(usize, usize)> = vec![(px, py)];
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
                    .and_then(|v| find_highest_gt(py, v))
                    .map(|ny| (px, ny)),
                Dir::E => self
                    .by_y
                    .get(&py)
                    .and_then(|v| find_highest_gt(px, v))
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
                    Dir::N => (px, 0),
                    Dir::S => (px, self.height),
                    Dir::E => (self.width, py),
                    Dir::W => (0, py),
                });
                break;
            }
            let (nx, ny) = next_spot.unwrap();
            coords.push((nx, ny));
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
        println!("{:?}", coords);
        Ok(Box::new(count_distinct_path(&coords)))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(Box::new(1))
    }
}

fn count_distinct_path(coords: &[(usize, usize)]) -> usize {
    // Walk the list of coordinates and count each unique (x,y) visited
    todo!()
}

fn find_highest_gt(px: usize, v: &[usize]) -> Option<usize> {
    v.iter().filter(|i| **i > px).max().map(|i| i - 1)
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
        solve.p1().unwrap();
    }
}
