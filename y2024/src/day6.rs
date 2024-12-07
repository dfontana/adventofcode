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
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.get_turns()
                .iter()
                .tuple_windows()
                .flat_map(|(c1, c2)| walk(c1, c2))
                .map(|(a, b, _)| (a, b))
                .collect::<HashSet<_>>()
                .len(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(self.p2s()))
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

impl Solve {
    fn get_turns(&self) -> Vec<(usize, usize, Dir)> {
        let (mut px, mut py, mut pd) = self.patrol.clone();
        let mut coords: Vec<(usize, usize, Dir)> = vec![(px, py, pd.clone())];
        loop {
            let next_spot = pd.next(&self.by_x, &self.by_y, px, py);
            if next_spot.is_none() {
                coords.push(pd.last(self.width, self.height, px, py));
                break;
            }
            let (nx, ny) = next_spot.unwrap();
            coords.push((nx, ny, pd.clone()));
            pd = pd.turn();
            px = nx;
            py = ny;
        }
        coords
    }

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
        let (mut px, mut py, mut pd) = self.patrol.clone();
        let mut coords: HashSet<(usize, usize, Dir)> =
            HashSet::from_iter(vec![(px, py, pd.clone())]);
        loop {
            let next_spot = pd.next(&by_x, &by_y, px, py);
            if next_spot.is_none() {
                return false;
            }
            let (nx, ny) = next_spot.unwrap();
            let nxt_pt = (nx, ny, pd.clone());
            if coords.contains(&nxt_pt) {
                return true;
            }
            coords.insert(nxt_pt);
            pd = pd.turn();
            px = nx;
            py = ny;
        }
    }

    // 792 -> too low
    fn p2s(&self) -> usize {
        let turns = self.get_turns();
        let turnset: HashSet<(usize, usize, Dir)> = HashSet::from_iter(turns.clone());
        // So if we put an object that causes us to return to the a spot in "turns" then we've made a loop
        // Naive option would be to walk the entire puzzle and at each step place a obstacle to see if it
        // hits another turn.

        // Better - take the paths walked; and every other "turn" point. See if any point on this path can make a perpendicular line to the other turn, such that it has the same origin direction.
        let items = turns
            .iter()
            .tuple_windows()
            .flat_map(|(t1, t2)| walk(t1, t2))
            .filter_map(|point| try_line(point, &turnset))
            .collect::<HashSet<_>>();
        // Filter items that don't loop but instead exit
        let items = items
            .iter()
            .filter(|item| self.triggers_loop(*item))
            .cloned()
            .collect::<HashSet<_>>();
        debug(&self.get_turns(), &items, self);
        items.len()
    }
}

fn debug(coords: &[(usize, usize, Dir)], itemset: &HashSet<(usize, usize)>, solve: &Solve) {
    let coordset = coords
        .iter()
        .map(|(a, b, _)| (a, b))
        .collect::<HashSet<_>>();
    let path = coords
        .iter()
        .tuple_windows()
        .flat_map(|(c1, c2)| walk(c1, c2))
        .map(|(a, b, _)| (a, b))
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
            } else if itemset.contains(&(x, y)) {
                print!("O");
            } else if coordset.contains(&(&x, &y)) {
                print!("+");
            } else if path.contains(&(x, y)) {
                print!(".");
            } else {
                print!("{}", i);
            }
        }
        println!();
    }
}

fn try_line(
    point: (usize, usize, Dir),
    turns: &HashSet<(usize, usize, Dir)>,
) -> Option<(usize, usize)> {
    // Can point form a perpendicular line to any turn?
    // Filter turns to those matching Dir.turn()
    // Then filter for any whose y == y (if point.dir == S|N) or x == x (if p.dir == W|E)
    let pd = point.2.turn();
    turns
        .iter()
        .any(|(x, y, d)| {
            *d == pd
                && match point.2 {
                    Dir::S | Dir::N => *y == point.1,
                    Dir::E | Dir::W => *x == point.0,
                }
        })
        .then_some(match point.2 {
            Dir::N => (point.0, point.1 - 1),
            Dir::S => (point.0, point.1 + 1),
            Dir::E => (point.0 + 1, point.1),
            Dir::W => (point.0 - 1, point.1),
        })
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
        assert_eq!(
            solve
                .p2()
                .map(|v| format!("{}", v).parse::<usize>().unwrap())
                .unwrap(),
            6
        );
    }
}
