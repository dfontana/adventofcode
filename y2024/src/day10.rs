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
    peaks: usize,
    trails: usize,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let map = Grid::new_from_map(value, |c| c.to_digit(10).unwrap());
        let (peaks, trails) = get_trail_heads(&map)
            .iter()
            .map(|loc| unique_trails_from(loc, &map))
            .fold((0, 0), |(mut peaks, mut trails), head_trails| {
                trails += head_trails.len();
                peaks += head_trails
                    .iter()
                    .filter_map(|t| t.last())
                    .collect::<HashSet<_>>()
                    .len();
                (peaks, trails)
            });
        Ok(Solve { peaks, trails })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(self.peaks))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(self.trails))
    }
}

fn get_trail_heads(map: &Grid<u32>) -> Vec<(usize, usize, &u32)> {
    map.iter().filter(|(_, _, height)| **height == 0).collect()
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
            if let Some((loc, h1)) = map.at_step(*y, *x, 1, &dir) {
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
