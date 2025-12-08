use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
struct Coord(usize, usize, usize, usize);
impl Coord {
    fn id(&self) -> usize {
        self.3
    }
    fn dist(&self, othe: &Coord) -> f64 {
        let dx = self.0.max(othe.0) - self.0.min(othe.0);
        let dy = self.1.max(othe.1) - self.1.min(othe.1);
        let dz = self.2.max(othe.2) - self.2.min(othe.2);
        (((dx * dx) + (dy * dy) + (dz * dz)) as f64).sqrt()
    }
}
pub struct Solve {
    pts: Vec<Coord>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            pts: value
                .trim()
                .lines()
                .enumerate()
                .map(|(id, v)| {
                    let mut vs = v.splitn(3, ",").map(|v| v.parse::<usize>().unwrap());
                    Coord(
                        vs.next().unwrap(),
                        vs.next().unwrap(),
                        vs.next().unwrap(),
                        id,
                    )
                })
                .collect(),
        })
    }
}

fn merge_clusters(
    members: &mut HashMap<usize, usize>,
    clusters: &mut HashMap<usize, HashSet<usize>>,
    i: &Coord,
    j: &Coord,
) {
    // Get all the members of the current cluster
    let i_c = members
        .get(&i.id())
        .and_then(|c| clusters.get(c))
        .unwrap()
        .clone();
    // Lookup the dest cluster
    let j_c_id = *members.get(&j.id()).unwrap();
    // Add them to the dest cluster
    clusters.get_mut(&j_c_id).unwrap().extend(i_c.iter());
    // Delete the old clusterIds not being orphaned
    // And point the current cluster's members to the dest cluster
    for id in &i_c {
        clusters.remove(members.get(id).unwrap());
        members.insert(*id, j_c_id);
    }
}

fn init_mappings(pts: &Vec<Coord>) -> (HashMap<usize, usize>, HashMap<usize, HashSet<usize>>) {
    let mut members: HashMap<usize, usize> = HashMap::new();
    let mut clusters: HashMap<usize, HashSet<usize>> = HashMap::new();
    for c in pts {
        members.insert(c.id(), c.id());
        clusters.insert(c.id(), HashSet::from_iter(vec![c.id()]));
    }
    (members, clusters)
}

fn link_until(
    pts: &Vec<Coord>,
    members: &mut HashMap<usize, usize>,
    clusters: &mut HashMap<usize, HashSet<usize>>,
    n: Option<usize>,
) -> (Coord, Coord) {
    let mut shortest_n = pts
        .iter()
        .flat_map(|i| {
            pts.iter()
                .filter(|j| j.id() != i.id())
                .map(move |j| (i.dist(j), i.clone(), j.clone()))
        })
        .map(|(d, i, j)| {
            if i.id() < j.id() {
                (d, i, j)
            } else {
                (d, j, i)
            }
        })
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut seen = HashSet::new();
    let mut ord = Vec::new();
    while let Some((_, i, j)) = shortest_n.next() {
        if let Some(v) = n
            && seen.len() == v
        {
            break;
        }
        if !seen.contains(&(i.clone(), j.clone())) {
            ord.push((i.clone(), j.clone()));
        }
        seen.insert((i.clone(), j.clone()));
    }

    let mut i = Coord::default();
    let mut j = Coord::default();
    for (ni, nj) in ord {
        if members.get(&ni.id()) == members.get(&nj.id()) {
            // Nothing to merge, in the same cluster
            continue;
        }
        merge_clusters(members, clusters, &ni, &nj);
        if clusters.len() == 1 {
            i = ni;
            j = nj;
        }
    }
    (i, j)
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let (mut members, mut clusters) = init_mappings(&self.pts);
        let _ = link_until(&self.pts, &mut members, &mut clusters, Some(1000));
        Ok(Box::new(
            clusters
                .values()
                .sorted_by_key(|c| c.len())
                .rev()
                .take(3)
                .fold(1, |a, e| a * e.len()),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let (mut members, mut clusters) = init_mappings(&self.pts);
        let (i, j) = link_until(&self.pts, &mut members, &mut clusters, None);
        Ok(Box::new(i.0 * j.0))
    }
}
