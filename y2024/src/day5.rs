use rust_util::Day;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub struct Solve {
    // Key must come before any values
    rules: HashMap<u32, HashSet<u32>>,
    // (middle item, PrintItem -> Items coming before it)
    prints: Vec<(Vec<u32>, HashMap<u32, HashSet<u32>>)>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (rulestr, printstr) = value.split_once("\n\n").unwrap();
        let rules = rulestr
            .lines()
            .filter_map(|l| l.split_once('|'))
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k)
                    .and_modify(|s: &mut HashSet<_>| {
                        s.insert(v);
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(v);
                        set
                    });
                acc
            });
        let prints = printstr
            .lines()
            .map(|l| l.split(',').filter_map(|i| i.parse::<u32>().ok()).collect())
            .map(|items: Vec<u32>| (items.clone(), make_before_set(items)))
            .collect();
        Ok(Solve { rules, prints })
    }
}

fn make_before_set(items: Vec<u32>) -> HashMap<u32, HashSet<u32>> {
    let mut map = HashMap::new();
    for (idx, item) in items.iter().enumerate() {
        map.insert(*item, HashSet::from_iter(items[..idx].iter().cloned()));
    }
    map
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        // Invalid prints are items whose "beforeValues" intersect with the associated rule's values.
        Ok(Box::new(
            self.prints
                .iter()
                .filter(|(_, p)| print_is_valid(p, &self.rules))
                .map(|(mi, _)| mi[mi.len() / 2])
                .sum::<u32>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        // Now find all the invalid pages (dropping valid ones)
        // Re-order them to be valid
        // sum their middle pages
        Ok(Box::new(1))
    }
}

fn print_is_valid(p: &HashMap<u32, HashSet<u32>>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    !p.iter().any(|(rk, vals)| {
        rules
            .get(rk)
            .and_then(|after_items| after_items.intersection(vals).next())
            .is_some()
    })
}
