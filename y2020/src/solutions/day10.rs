use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;
use std::{
  collections::VecDeque,
  fmt,
};

pub struct Solve {
  jolts: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      jolts: read_input(d)?
        .lines()
        .map(|i| i.parse::<i32>())
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(p1_mut(&self.jolts).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(build_tree(&self.jolts).to_string())
  }
}

fn p1_mut(jolts: &Vec<i32>) -> i32 {
  // ~200-300us
  let mut copy = jolts.clone();
  copy.sort();
  copy.push(copy[copy.len() - 1] + 3);
  let mut c1 = 0;
  let mut c3 = 0;
  let mut prev = 0;
  for i in copy.iter() {
    match i - prev {
      1 => c1 += 1,
      3 => c3 += 1,
      _ => (),
    }
    prev = *i;
  }
  c1 * c3
}

fn build_tree(jolts: &Vec<i32>) -> i32 {
  // 1. Sort input to process children efficiently
  let mut copy = jolts.clone();
  copy.sort();
  copy.push(copy[copy.len() - 1] + 3);

  // 2. Create a queue, to reduce the search space
  // TODO can I just use an iterator & advance it when needed?
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.extend(copy.iter());

  // 3. Prepare our tree
  let mut tree: ArenaTree<i32> = ArenaTree::default();
  let idx = tree.add_node(0);

  // 4. Init frontier. These are nodes in the tree.
  let mut frontier: VecDeque<usize> = VecDeque::new();
  frontier.push_front(idx);

  // 5. Build Tree
  while let Some(exp) = frontier.pop_front() {
    let me_val = tree.arena[exp].val;
    let me_idx = tree.arena[exp].idx;

    // Clear items from queue that are no longer valid
    let min_to_keep = frontier
      .iter()
      .map(|f| tree.arena[*f].val)
      .min()
      .unwrap_or(me_val);
    while let Some(old) = queue.pop_front() {
      if old >= min_to_keep {
        queue.push_front(old);
        break;
      }
    }

    println!("{:?}", frontier.len());

    // Push items into children that are potential paths to take
    let mut peek = queue.iter();
    while let Some(next) = peek.next().filter(|f| **f <= me_val + 3) {
      if *next <= me_val {
        continue;
      }
      // Add to me children (lol) & frontier
      let idx = tree.add_node(*next);
      tree.arena[me_idx].children.push(idx);
      frontier.push_back(idx);
    }
  }
  // println!("{}", tree);
  tree.leaves()
}

#[derive(Debug)]
struct Node<T>
where
  T: PartialEq,
{
  idx: usize,
  val: T,
  parent: Option<usize>,
  children: Vec<usize>,
}

impl<T> Node<T>
where
  T: PartialEq,
{
  fn new(idx: usize, val: T) -> Self {
    Self {
      idx,
      val,
      parent: None,
      children: vec![],
    }
  }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
  T: PartialEq,
{
  arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
  T: PartialEq,
{
  fn add_node(&mut self, val: T) -> usize {
    let idx = self.arena.len();
    self.arena.push(Node::new(idx, val));
    idx
  }

  fn leaves(&self) -> i32 {
    if self.arena.is_empty() {
      return 0;
    }
    fn count_leaves<T>(arena: &Vec<Node<T>>, idx: usize) -> i32
    where
      T: PartialEq,
    {
      if arena[idx].children.is_empty() {
        return 1;
      }
      arena[idx]
        .children
        .iter()
        .map(|c| count_leaves(arena, *c))
        .sum()
    };
    count_leaves(&self.arena, 0)
  }
}

impl<T> fmt::Display for ArenaTree<T>
where
  T: fmt::Display + PartialEq,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    pprint_tree(f, &self.arena, 0, "".to_string(), true)
  }
}
fn pprint_tree<T>(
  f: &mut fmt::Formatter<'_>,
  arena: &Vec<Node<T>>,
  idx: usize,
  prefix: String,
  last: bool,
) -> fmt::Result
where
  T: fmt::Display + PartialEq,
{
  let node = &arena[idx];
  let prefix_current = if last { "`- " } else { "|- " };

  write!(f, "{}{}{}\n", prefix, prefix_current, node.val)?;

  let prefix_child = if last { "   " } else { "|  " };
  let prefix = prefix + prefix_child;

  if !node.children.is_empty() {
    let last_child = node.children.len() - 1;
    for (i, child) in node.children.iter().enumerate() {
      pprint_tree(f, arena, *child, prefix.to_string(), i == last_child)?
    }
  }

  Ok(())
}
