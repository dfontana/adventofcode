use rust_util::{
    grid::{Dir, Grid},
    Day,
};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

pub struct Solve {
    map: Grid<u32>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            map: Grid::new_from_map(value, |c| c.to_digit(10).unwrap()),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            get_trail_heads(&self.map)
                .iter()
                .map(|loc| reachable_peaks_from(loc, &self.map).len())
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            get_trail_heads(&self.map)
                .iter()
                .map(|loc| unique_trails_from(loc, &self.map).len())
                .sum::<usize>(),
        ))
    }
}

fn get_trail_heads(map: &Grid<u32>) -> Vec<(usize, usize, &u32)> {
    map.iter().filter(|(_, _, height)| **height == 0).collect()
}

fn reachable_peaks_from(loc: &(usize, usize, &u32), map: &Grid<u32>) -> HashSet<(usize, usize)> {
    // BFS from loc, find all the unique loc's with height == 9
    let mut ret = HashSet::new();
    let mut frontier = VecDeque::from_iter(vec![(loc.0, loc.1, *loc.2)]);
    while let Some((y, x, h)) = frontier.pop_front() {
        for dir in [Dir::E, Dir::W, Dir::N, Dir::S] {
            if let Some((loc, h1)) = map.at_step(y, x, 1, dir) {
                if *h1 != h + 1 {
                    // not a valid step to take
                    continue;
                }
                if *h1 == 9 {
                    // Goal found, push to results
                    ret.insert(loc);
                    continue;
                }
                // Otherwise we can step this way on our hike
                frontier.push_back((loc.0, loc.1, *h1));
            }
        }
    }
    ret
}

fn unique_trails_from(
    loc: &(usize, usize, &u32),
    map: &Grid<u32>,
) -> HashSet<Vec<(usize, usize, u32)>> {
    // BFS from loc, find all the unique loc's with height == 9
    let mut ret = HashSet::new();
    let mut frontier = VecDeque::from_iter(vec![vec![(loc.0, loc.1, *loc.2)]]);
    while let Some(path) = frontier.pop_front() {
        let (y, x, h) = path.last().unwrap(); // Never empty
        for dir in [Dir::E, Dir::W, Dir::N, Dir::S] {
            if let Some((loc, h1)) = map.at_step(*y, *x, 1, dir) {
                if *h1 != h + 1 {
                    // not a valid step to take
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push((loc.0, loc.1, *h1));
                if *h1 == 9 {
                    // Goal found, push to results
                    ret.insert(new_path);
                    continue;
                } else {
                    // Otherwise we can step this way on our hike
                    frontier.push_back(new_path);
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            36
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            81
        );
    }
}
