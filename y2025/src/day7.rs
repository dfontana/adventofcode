use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

pub struct Solve {
    // y, x
    root: (usize, usize),
    // x -> vec<y> (sorted)
    sps: HashMap<usize, Vec<usize>>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pts: Vec<(usize, usize, char)> = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (y, x, c)))
            .filter(|c| c.2 == 'S' || c.2 == '^')
            .collect();
        let root = pts.iter().find(|c| c.2 == 'S').map(|c| (c.0, c.1)).unwrap();
        let mut sps = HashMap::new();
        for (y, x, _) in pts.iter() {
            sps.entry(*x)
                .and_modify(|v: &mut Vec<usize>| v.push(*y))
                .or_insert(vec![*y]);
        }
        for (_, v) in sps.iter_mut() {
            v.sort();
        }
        Ok(Solve { root, sps })
    }
}

impl Solve {
    /// From the given coordinate find the next split(s) in the beam, if any at all
    fn maybe_split(
        &self,
        (y, x): (usize, usize),
    ) -> Option<((usize, usize), Option<(usize, usize)>, (usize, usize))> {
        let next = self
            .sps
            .get(&x)
            .and_then(|ys| ys.iter().filter(|v| **v > y).next())?;
        let left = x.checked_sub(1).map(|nx| (*next, nx));
        let right = (*next, x + 1);
        Some(((*next, x), left, right))
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut splits = 0;
        let mut hit_splits = HashSet::new();
        let mut frontier = vec![self.root];
        while let Some((y, x)) = frontier.pop() {
            if let Some((sp, l, r)) = self.maybe_split((y, x)) {
                if hit_splits.contains(&sp) {
                    continue;
                }
                hit_splits.insert(sp);
                splits += 1;
                if let Some(n) = l {
                    frontier.push(n);
                }
                frontier.push(r);
            }
        }
        Ok(Box::new(splits))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
        let total_paths = count(&self, &mut memo, self.root);
        Ok(Box::new(total_paths))
    }
}

fn count(ctx: &Solve, memo: &mut HashMap<(usize, usize), usize>, (y, x): (usize, usize)) -> usize {
    if let Some(n) = memo.get(&(y, x)) {
        return *n;
    }
    match ctx.maybe_split((y, x)) {
        None => 1,
        Some((_, l, r)) => {
            let mut subpaths = 0;
            if let Some(n) = l {
                subpaths += count(ctx, memo, n);
            }
            subpaths += count(ctx, memo, r);
            memo.insert((y, x), subpaths);
            subpaths
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            21
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            40
        );
    }
}
