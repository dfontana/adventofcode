use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

type NetId = String;
pub struct Solve {
    adjs: HashMap<NetId, HashSet<NetId>>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            adjs: value.lines().filter_map(|l| l.split_once("-")).fold(
                HashMap::new(),
                |mut acc, x| {
                    acc.entry(x.0.to_string())
                        .and_modify(|y| {
                            y.insert(x.1.to_string());
                        })
                        .or_insert_with(|| {
                            let mut e = HashSet::new();
                            e.insert(x.1.to_string());
                            e
                        });
                    acc
                },
            ),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(t_triplets(&self.adjs)))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(largest_clique(&self.adjs)))
    }
}

// This algo just wants to know if computers are connected directly or
// indirectly to form a triplet of connections. These connections must
// involve a computer with 't'
fn t_triplets(adjs: &HashMap<NetId, HashSet<NetId>>) -> usize {
    let mut seen = HashSet::new();
    for (id, adj) in adjs.iter() {
        for o1 in adj.iter() {
            let Some(o1_adj) = adjs.get(o1) else {
                continue;
            };
            for o2 in o1_adj {
                if !id.starts_with("t") && !o1.starts_with("t") && !o2.starts_with("t") {
                    continue;
                }
                if is_connected(adjs, id, o2) {
                    let mut triplet = [id, o1, o2];
                    triplet.sort();
                    seen.insert(triplet);
                }
            }
        }
    }
    seen.len()
}

fn largest_clique(adjs: &HashMap<NetId, HashSet<NetId>>) -> String {
    let all_u_points: HashSet<NetId> = adjs
        .iter()
        .flat_map(|(k, v)| std::iter::once(k).chain(v.iter()))
        .map(|s| s.to_string())
        .collect();
    let all_points = Vec::from_iter(all_u_points);
    find_largest(adjs, &all_points, Vec::new())
        .iter()
        .sorted()
        .join(",")
}

fn find_largest(
    adjs: &HashMap<NetId, HashSet<NetId>>,
    rem: &[NetId],
    clique: Vec<NetId>,
) -> Vec<NetId> {
    let mut largest = clique.clone();
    for (i, p) in rem.iter().enumerate() {
        if is_clique(adjs, &clique, p) {
            let mut n_clique = clique.clone();
            n_clique.push(p.to_string());
            let o_clique = find_largest(adjs, &rem[i + 1..], n_clique);
            if o_clique.len() > largest.len() {
                largest = o_clique;
            }
        }
    }
    largest
}

fn is_clique(adjs: &HashMap<NetId, HashSet<NetId>>, clique: &Vec<NetId>, p: &NetId) -> bool {
    clique.iter().all(|c| is_connected(adjs, c, p))
}

fn is_connected(adjs: &HashMap<NetId, HashSet<NetId>>, a: &NetId, b: &NetId) -> bool {
    adjs.get(a).filter(|v| v.contains(b)).is_some()
        || adjs.get(b).filter(|v| v.contains(a)).is_some()
}
