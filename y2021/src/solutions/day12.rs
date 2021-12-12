use rust_util::{AocDay, Day};
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

pub struct Solve {
  nodes: Arena,
}

type NodeId = String;
type Arena = HashMap<NodeId, Node>;

#[derive(Clone, Debug)]
struct Node {
  id: NodeId,
  edges: HashSet<NodeId>,
  small: bool,
  terminal: bool,
}

impl Node {
  fn add(&mut self, id: NodeId) {
    self.edges.insert(id);
  }
}

#[derive(Clone, Debug)]
struct Path {
  top: NodeId,
  up_to: Vec<NodeId>,
  used_double: bool,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      nodes: rust_util::read_input(2021, d)?
        .lines()
        .flat_map(|line| {
          let mut pair = line.splitn(2, "-");
          let src = pair.next().unwrap();
          let dst = pair.next().unwrap();
          vec![(src.clone(), dst.clone()), (dst.clone(), src.clone())]
        })
        .fold(HashMap::new(), |mut nodes, (s, d)| {
          nodes
            .entry(s.to_string())
            .and_modify(|node| node.add(d.to_string()))
            .or_insert(Node {
              id: s.to_string(),
              edges: HashSet::from_iter(vec![d.to_string()]),
              small: s.to_lowercase() == s,
              terminal: s == "end",
            });
          nodes
        }),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      explore(&self.nodes, &mut init_frontier(&self.nodes, true)).len(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      explore(&self.nodes, &mut init_frontier(&self.nodes, false)).len(),
    ))
  }
}

fn init_frontier(nodes: &Arena, used_double: bool) -> Vec<Path> {
  nodes
    .get("start")
    .unwrap()
    .edges
    .iter()
    .map(|e| Path {
      top: e.clone(),
      up_to: vec!["start".to_owned()],
      used_double,
    })
    .collect()
}

fn explore(nodes: &Arena, frontier: &mut Vec<Path>) -> Vec<Path> {
  let mut paths: Vec<Path> = Vec::new();
  while let Some(path) = frontier.pop() {
    let node = nodes.get(&path.top).unwrap();
    if node.terminal {
      paths.push(path.clone());
      continue;
    }
    if node.small && path.up_to.contains(&node.id) && path.used_double {
      // Small nodes may be visited once or twice, so skip if we have
      continue;
    }
    node
      .edges
      .iter()
      .filter(|nid| *nid != "start")
      .for_each(|nid| {
        let mut nw_path = path.clone();
        nw_path.top = nid.clone();
        nw_path.up_to.push(node.id.clone());
        if node.small && path.up_to.contains(&node.id) && !path.used_double {
          nw_path.used_double = true
        }
        frontier.push(nw_path);
      });
  }
  paths
}
