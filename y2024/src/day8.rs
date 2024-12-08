use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Sat(char, i64, i64);
pub struct Solve {
    // By-frequency
    sats: HashMap<char, Vec<Sat>>,
    bounds: (i64, i64),
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let width = value.lines().nth(0).map(|v| v.len()).ok_or("Bad input")? as i64;
        let height = value.lines().count() as i64;
        Ok(Solve {
            sats: value
                .lines()
                .enumerate()
                .flat_map(|(x, v)| {
                    v.chars()
                        .enumerate()
                        .filter(|(_, v)| *v != '.')
                        .map(move |(y, f)| Sat(f, x as i64, y as i64))
                })
                .fold(HashMap::new(), |mut acc, x| {
                    acc.entry(x.0)
                        .and_modify(|v: &mut Vec<_>| v.push(x.clone()))
                        .or_insert_with(|| vec![x]);
                    acc
                }),
            bounds: (width, height),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            anti_nodes(self, find_anti_nodes)
                .collect::<HashSet<_>>()
                .len(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            anti_nodes(self, find_harmonic_nodes)
                .collect::<HashSet<_>>()
                .len(),
        ))
    }
}

fn anti_nodes<'a, F>(solve: &'a Solve, extrapolate: F) -> impl Iterator<Item = Sat> + use<'a, F>
where
    F: Fn(&Sat, &Sat, (i64, i64)) -> Vec<Sat> + 'a,
{
    solve
        .sats
        .iter()
        .flat_map(|(_, v)| v.iter().tuple_combinations())
        .flat_map(move |(s1, s2)| extrapolate(s1, s2, solve.bounds))
}

fn find_anti_nodes(s1: &Sat, s2: &Sat, bounds: (i64, i64)) -> Vec<Sat> {
    vec![
        harmonics(s1, s2, bounds).nth(2),
        harmonics(s2, s1, bounds).nth(2),
    ]
    .into_iter()
    .flatten()
    .collect_vec()
}

fn find_harmonic_nodes(s1: &Sat, s2: &Sat, bounds: (i64, i64)) -> Vec<Sat> {
    harmonics(s1, s2, bounds).collect_vec()
}

fn harmonics(s1: &Sat, s2: &Sat, bounds: (i64, i64)) -> impl Iterator<Item = Sat> {
    SatIter {
        deltas: (s2.1 - s1.1, s2.2 - s1.2),
        bounds,
        flipped_deltas: false,
        root: Sat('#', s1.1, s1.2),
        next: Some(Sat('#', s1.1, s1.2)),
    }
}

struct SatIter {
    deltas: (i64, i64),
    bounds: (i64, i64),
    flipped_deltas: bool,
    root: Sat,
    next: Option<Sat>,
}

impl SatIter {
    fn step_from(&self, sat: &Sat) -> Option<Sat> {
        let sx = sat.1 + self.deltas.0;
        let sy = sat.2 + self.deltas.1;
        if sx < 0 || sx >= self.bounds.0 || sy < 0 || sy >= self.bounds.1 {
            None
        } else {
            Some(Sat('#', sx, sy))
        }
    }
}

impl Iterator for SatIter {
    type Item = Sat;

    fn next(&mut self) -> Option<Self::Item> {
        let sat = self.next.clone()?;
        self.next = match self.step_from(&sat) {
            Some(v) => Some(v),
            None if !self.flipped_deltas => {
                // Wrap back to root and step other way.
                self.flipped_deltas = true;
                self.deltas = (-self.deltas.0, -self.deltas.1);
                self.step_from(&self.root)
            }
            None => None,
        };
        Some(sat)
    }
}
