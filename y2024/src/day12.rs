use rust_util::Day;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

pub struct Solve {
    layers: HashMap<char, SparseGrid>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            layers: into_layers(value),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.layers
                .values()
                .flat_map(|l| find_beds_of_layer(l))
                .map(|bed| area_and_perimeter_of_bed(&bed))
                .map(|(a, p)| a * p)
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.layers
                .values()
                .flat_map(|l| find_beds_of_layer(l))
                .map(|bed| area_and_corners_of_bed(&bed))
                .map(|(a, p)| a * p)
                .sum::<usize>(),
        ))
    }
}

type Loc = (usize, usize);
type SparseGrid = HashSet<Loc>;
type Bed = HashSet<Loc>;
fn into_layers(input: String) -> HashMap<char, SparseGrid> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| ((y, x), c)))
        .fold(HashMap::new(), |mut acc, (loc, c)| {
            acc.entry(c)
                .and_modify(|hs| {
                    hs.insert(loc);
                })
                .or_insert_with(|| HashSet::from_iter(vec![loc]));
            acc
        })
}

fn find_beds_of_layer(grid: &SparseGrid) -> Vec<Bed> {
    let mut beds: Vec<Bed> = Vec::new();
    let mut global_seen: HashSet<Loc> = HashSet::new();
    for loc in grid.iter() {
        if global_seen.contains(loc) {
            // It's in a bed already
            continue;
        }
        // Iterate repeatedly growing from point until no more growth; Bed is filled.
        let mut bed: Bed = HashSet::new();
        let mut frontier = VecDeque::from_iter(vec![*loc]);
        while let Some((y, x)) = frontier.pop_front() {
            if bed.contains(&(y, x)) {
                continue;
            }
            bed.insert((y, x));
            if grid.contains(&(y - 1, x)) {
                frontier.push_back((y - 1, x));
            }
            if grid.contains(&(y + 1, x)) {
                frontier.push_back((y + 1, x));
            }
            if grid.contains(&(y, x - 1)) {
                frontier.push_back((y, x - 1));
            }
            if grid.contains(&(y, x + 1)) {
                frontier.push_back((y, x + 1));
            }
        }
        global_seen.extend(bed.iter());
        beds.push(bed);
    }
    beds
}

fn area_and_perimeter_of_bed(bed: &Bed) -> (usize, usize) {
    // Area is just the size of the bed
    // Perimeter == (area*4) - (non-unique number of edges touching)
    let area = bed.len();
    let mut touching = 0;
    for (y, x) in bed.iter() {
        if bed.contains(&(y - 1, *x)) {
            touching += 1;
        }
        if bed.contains(&(y + 1, *x)) {
            touching += 1;
        }
        if bed.contains(&(*y, x - 1)) {
            touching += 1;
        }
        if bed.contains(&(*y, x + 1)) {
            touching += 1;
        }
    }
    (area, area * 4 - touching)
}

// 423883 -> Low
fn area_and_corners_of_bed(bed: &Bed) -> (usize, usize) {
    let area = bed.len();
    let corners = bed
        .iter()
        .filter(|(y, x)| {
            // If 3 not 4 are touching, it's interior corner
            // if 2 not 4 touching it's exterior corner
            let mut touching = 0;
            if bed.contains(&(y - 1, *x)) {
                touching += 1;
            }
            if bed.contains(&(y + 1, *x)) {
                touching += 1;
            }
            if bed.contains(&(*y, x - 1)) {
                touching += 1;
            }
            if bed.contains(&(*y, x + 1)) {
                touching += 1;
            }
            touching == 3 || touching == 2
        })
        .count()
        / 2;
    (area, corners)
}
