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
  explored: HashSet<NodeId>,
  used_double: bool,
}

fn set(v: NodeId) -> HashSet<NodeId> {
  let mut set = HashSet::new();
  set.insert(v);
  set
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    let mut nodes: Arena = HashMap::new();
    for line in input.lines() {
      let mut pair = line.splitn(2, "-");
      let src = pair.next().unwrap().to_string();
      let dst = pair.next().unwrap().to_string();
      // Add the src node + src -> dst conns
      nodes
        .entry(src.to_string())
        .and_modify(|node| node.add(dst.to_string()))
        .or_insert(Node {
          id: src.to_string(),
          edges: set(dst.to_string()),
          small: src.to_lowercase() == src,
          terminal: src == "end",
        });

      // Don't forget to register a dst node (as long as it's not start)
      nodes
        .entry(dst.to_string())
        .and_modify(|node| node.add(src.to_string()))
        .or_insert(Node {
          id: dst.to_string(),
          edges: set(src.to_string()),
          small: dst.to_lowercase() == dst,
          terminal: dst == "end",
        });
    }
    Ok(Box::new(Solve { nodes }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut paths: Vec<Path> = Vec::new();
    let mut frontier: Vec<Path> = self
      .nodes
      .get("start")
      .unwrap()
      .edges
      .iter()
      .map(|e| Path {
        top: e.clone(),
        up_to: vec!["start".to_owned()],
        explored: set("start".into()),
        used_double: false,
      })
      .collect();
    while let Some(mut crt_path) = frontier.pop() {
      let next = crt_path.top.clone();
      let node = self.nodes.get(&next).unwrap();
      if node.terminal {
        paths.push(crt_path.clone());
        continue;
      }

      // If you are already explored, skip
      if crt_path.explored.contains(&next) {
        continue;
      }

      // Push self to explored, but only if we're small. Everyone else can be visited.
      if node.small {
        crt_path.explored.insert(next.clone());
      }

      // Otherwise push all your edges onto the frontier as new paths
      node.edges.iter().for_each(|nid| {
        if nid == "start" {
          return;
        }
        let mut path = crt_path.clone();
        path.top = nid.clone();
        path.up_to.push(next.clone());
        frontier.push(path);
      });
    }
    Ok(Box::new(paths.len()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut paths: Vec<Path> = Vec::new();
    let mut frontier: Vec<Path> = self
      .nodes
      .get("start")
      .unwrap()
      .edges
      .iter()
      .map(|e| Path {
        top: e.clone(),
        up_to: vec!["start".to_owned()],
        explored: set("start".into()),
        used_double: false,
      })
      .collect();
    while let Some(mut crt_path) = frontier.pop() {
      let next = crt_path.top.clone();
      let node = self.nodes.get(&next).unwrap();
      if node.terminal {
        paths.push(crt_path.clone());
        continue;
      }

      // If you are already explored, skip
      if crt_path.explored.contains(&next) && crt_path.used_double {
        continue;
      }
      let double_used = crt_path.explored.contains(&next) && !crt_path.used_double;

      // Push self to explored, but only if we're small. Everyone else can be visited.
      if node.small {
        crt_path.explored.insert(next.clone());
      }

      // Otherwise push all your edges onto the frontier as new paths
      node.edges.iter().for_each(|nid| {
        if nid == "start" {
          return;
        }
        let mut path = crt_path.clone();
        path.top = nid.clone();
        path.up_to.push(next.clone());
        if node.small && double_used {
          path.used_double = true
        }
        frontier.push(path);
      });
    }
    Ok(Box::new(paths.len()))
  }
}
