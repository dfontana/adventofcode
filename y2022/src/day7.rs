use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
  path::PathBuf,
};

#[derive(Clone, Debug)]
enum DirSized {
  Raw(usize),
  Computed(usize),
}

pub struct Solve {
  path_sizes: HashMap<PathBuf, DirSized>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut path_sizes: HashMap<PathBuf, DirSized> = HashMap::new();
    let mut parents: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();
    let mut current_path = PathBuf::new();
    for line in value.lines() {
      let mut parts = line.splitn(3, ' ');
      match (parts.next(), parts.next(), parts.next()) {
        (Some("$"), Some("cd"), Some("..")) => {
          current_path.pop();
        }
        (Some("$"), Some("cd"), Some("/")) => {
          current_path = PathBuf::new();
          current_path.push("/");
        }
        (Some("$"), Some("cd"), Some(dir)) => {
          current_path.push(dir);
        }
        (Some("$"), Some("ls"), None) => {
          continue;
        }
        (Some("dir"), Some(folder), None) => {
          let mut path_to = current_path.clone();
          path_to.push(folder);
          parents
            .entry(current_path.clone())
            .and_modify(|v| {
              v.insert(path_to.clone());
            })
            .or_insert_with(|| {
              let mut set = HashSet::new();
              set.insert(path_to);
              set
            });
          path_sizes
            .entry(current_path.clone())
            .and_modify(|d| {
              if let DirSized::Computed(v) = d {
                *d = DirSized::Raw(*v);
              }
            })
            .or_insert(DirSized::Raw(0));
        }
        (Some(size), Some(_), None) => {
          let parsed = size.parse::<usize>().unwrap();
          path_sizes
            .entry(current_path.clone())
            .and_modify(|d| match d {
              DirSized::Raw(v) | DirSized::Computed(v) => {
                *v += parsed;
              }
            })
            .or_insert(DirSized::Computed(parsed));
        }
        _ => unreachable!("Unknown pattern hit"),
      }
    }

    for path in path_sizes.clone().keys() {
      sum_children(&mut path_sizes, &parents, path);
    }

    Ok(Solve { path_sizes })
  }
}

fn sum_children(
  sizes: &mut HashMap<PathBuf, DirSized>,
  parents: &HashMap<PathBuf, HashSet<PathBuf>>,
  being_summed: &PathBuf,
) {
  if let Some(DirSized::Computed(_)) = sizes.get(being_summed) {
    return;
  }
  let mut total: usize = 0;
  for child in parents.get(being_summed).unwrap() {
    sum_children(sizes, parents, child);
    if let Some(DirSized::Computed(sz)) = sizes.get(child) {
      total += sz;
    }
  }
  sizes.entry(being_summed.clone()).and_modify(|d| {
    if let DirSized::Raw(v) = d {
      *d = DirSized::Computed(*v + total)
    }
  });
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut total: usize = 0;
    for size in self.path_sizes.values() {
      if let DirSized::Computed(v) = size {
        if *v <= 100000 {
          total += v;
        }
        continue;
      }
    }
    Ok(Box::new(total))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut smallest = PathBuf::new();
    smallest.push("/");
    let mut smallest_size: usize = 70_000_000;
    let used = match self.path_sizes.get(&smallest).unwrap() {
      DirSized::Computed(v) | DirSized::Raw(v) => *v,
    };
    let space_needed = 30_000_000 - (70_000_000 - used);
    for (path, size) in &self.path_sizes {
      let sz = match size {
        DirSized::Computed(v) | DirSized::Raw(v) => *v,
      };
      if sz >= space_needed && sz < smallest_size {
        smallest_size = sz;
        smallest = path.to_path_buf();
      }
    }
    Ok(Box::new(format!(
      "{} - {}",
      smallest.to_string_lossy(),
      smallest_size
    )))
  }
}
