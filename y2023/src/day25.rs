use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};
use rust_util::Day;
use std::{
  collections::{HashMap, HashSet, VecDeque},
  error::Error,
  fmt::Display,
};

pub struct Solve {
  input: Graph,
  nodes: HashSet<String>,
}

#[derive(Clone)]
struct Graph(HashMap<String, Vec<String>>);

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut input: HashMap<String, Vec<String>> = HashMap::new();
    for l in value.lines() {
      let (k, vs) = l.split_once(": ").unwrap();
      let vals = vs.split_whitespace().map(|s| s.to_string()).collect_vec();
      vals.iter().for_each(|v| {
        input
          .entry(v.to_string())
          .and_modify(|v| {
            v.push(k.to_string());
          })
          .or_insert(vec![k.to_string()]);
      });
      input
        .entry(k.to_string())
        .and_modify(|v| v.extend(vals.iter().map(|s| s.clone())))
        .or_insert(vals);
    }
    let mut nodes = HashSet::from_iter(input.keys().map(|s| s.clone()));
    input.values().flat_map(|v| v.iter()).for_each(|v| {
      nodes.insert(v.clone());
    });
    Ok(Solve {
      nodes,
      input: Graph(input),
    })
  }
}

impl Solve {
  fn sample_nodes(&self, rng: &mut ThreadRng) -> (&str, &str) {
    let spl = self.nodes.iter().choose_multiple(rng, 2);
    (spl[0], spl[1])
  }
}

impl Graph {
  fn path<'a>(&'a self, a: &'a str, b: &str) -> Vec<&str> {
    let mut frontier = VecDeque::from_iter(vec![(a, HashSet::from_iter(vec![a]))]);
    while let Some((n, path)) = frontier.pop_front() {
      if n == b {
        return path.iter().map(|v| *v).collect_vec();
      }
      let Some(vs) = self.0.get(n) else {
        continue;
      };
      vs.iter()
        .filter(|i| !path.contains(&i.as_str()))
        .for_each(|v| {
          let mut np: HashSet<&str> = path.clone();
          np.insert(v);
          frontier.push_back((v, np));
        });
    }
    unreachable!()
  }

  fn find_edges<'a>(&'a self, solve: &'a Solve) -> Vec<(&'a str, &'a str)> {
    let mut seen: HashMap<(&str, &str), usize> = HashMap::new();
    let mut rng = thread_rng();
    for i in 0..1000 {
      let (a, b) = solve.sample_nodes(&mut rng);
      println!("[{}] exploring {},{}", i, a, b);
      self
        .path(a, b)
        .iter()
        .tuple_windows::<(_, _)>()
        .for_each(|(a, b)| {
          seen.entry((*a, *b)).and_modify(|v| *v += 1).or_insert(1);
        });
    }
    seen
      .iter()
      .sorted_by_key(|(_, v)| **v)
      .rev()
      .take(3)
      .map(|(k, _)| *k)
      .collect()
  }

  fn disconnect(&mut self, (a, b): &(&str, &str)) {
    let mut avs = self.0.get_mut(*a).unwrap();
    avs.retain(|v| v != b);
    let mut bvs = self.0.get_mut(*b).unwrap();
    bvs.retain(|v| v != a);
  }

  fn group_size(&self, a: &str) -> usize {
    let mut seen = HashSet::new();
    let mut frontier = VecDeque::from_iter(vec![a]);
    while let Some(n) = frontier.pop_front() {
      if seen.contains(n) {
        continue;
      }
      seen.insert(n);
      let Some(vs) = self.0.get(n) else {
        continue;
      };
      vs.iter().for_each(|v| {
        frontier.push_back(v);
      });
    }
    seen.len()
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let edges = self.input.find_edges(&self);
    println!("Found: {:?}", edges);
    let mut graph = self.input.clone();
    edges.iter().for_each(|e| graph.disconnect(e));
    let g1 = graph.group_size(edges[0].0);
    let g2 = graph.group_size(edges[0].1);
    Ok(Box::new(g1 * g2))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
