use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

type Img = Vec<String>;
type IdOrient = (usize, u8);

#[derive(Clone)]
struct Tile {
  id: IdOrient,
  img: Img,
}

#[derive(Clone)]
struct Stitch {
  id: IdOrient,
  l: Option<Box<Stitch>>,
  r: Option<Box<Stitch>>,
  u: Option<Box<Stitch>>,
  d: Option<Box<Stitch>>,
}

enum State {
  Finished,
  Dead,
  Explore(Vec<Stitch>),
}

pub struct Solve {
  tiles: Vec<Tile>,
  side_len: usize,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let tiles = read_input(d)?
      .split("\n\n")
      .map(|t| {
        let mut lines = t.lines();
        let id: usize = lines
          .next()
          .unwrap()
          .trim_start_matches("Tile ")
          .trim_end_matches(':')
          .parse()
          .unwrap();
        let img = lines.map(str::to_owned).collect();
        Tile { img, id: (id, 0) }
      })
      .collect::<Vec<_>>();
    Ok(Solve {
      tiles: tiles
        .iter()
        .map(|t| get_all_orientations(&t.img, t.id.0))
        .flatten()
        .collect(),
      side_len: (tiles.len() as f64).sqrt() as usize,
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let root = self.tiles[0].id;
    let images = self.tiles.iter().fold(HashMap::new(), |mut acc, t| {
      acc.insert(t.id, t.clone());
      acc
    });
    let s = match explore(root, &images) {
      Some(s) => s,
      None => return Err("No ans found".into()),
    };
    s.print();
    Ok("Use Printout :P".to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

impl Stitch {
  fn empty(id: IdOrient) -> Stitch {
    Stitch {
      id,
      l: None,
      r: None,
      u: None,
      d: None,
    }
  }

  fn apply<F>(&self, mut fnc: F)
  where
    F: FnMut(&Stitch),
  {
    let mut ptr = self;
    loop {
      if let Some(n) = &ptr.l {
        ptr = n.as_ref();
        fnc(ptr);
        continue;
      }
      if let Some(n) = &ptr.r {
        ptr = n.as_ref();
        fnc(ptr);
        continue;
      }
      if let Some(n) = &ptr.u {
        ptr = n.as_ref();
        fnc(ptr);
        continue;
      }
      if let Some(n) = &ptr.d {
        ptr = n.as_ref();
        fnc(ptr);
        continue;
      }
      break;
    }
  }

  fn all_in_path(&self) -> Vec<IdOrient> {
    let mut path = Vec::new();
    self.apply(|ptr| path.push(ptr.id));
    path
  }

  fn print(&self) {
    self.apply(|ptr| {
      print!("\n[ID] {} - ", ptr.id.0);
      if let Some(n) = &ptr.l {
        print!("U:{} ", n.id.0);
      }
      if let Some(n) = &ptr.r {
        print!("U:{} ", n.id.0);
      }
      if let Some(n) = &ptr.u {
        print!("U:{} ", n.id.0);
      }
      if let Some(n) = &ptr.d {
        print!("U:{} ", n.id.0);
      }
      println!();
    });
  }
}

fn explore(root: IdOrient, images: &HashMap<IdOrient, Tile>) -> Option<Stitch> {
  let mut ans: Option<Stitch> = None;
  let mut frontier: VecDeque<Stitch> = VecDeque::new();
  frontier.push_back(Stitch::empty(root));
  while let Some(next) = frontier.pop_front() {
    // Check if any unseen images can fit onto next.
    // Enqueue them for exploration
    match expand(&next, &images) {
      // No images left for next, all got used
      // and it's a square
      State::Finished => {
        ans = Some(next);
        break;
      }
      // There's options that fit, so let's add them on
      // to explore further
      State::Explore(expanse) => {
        expanse.iter().for_each(|f| frontier.push_back(f.clone()));
      }
      // Nothing fits onto the branch
      // Or it wasn't a square (when images are left)
      State::Dead => (),
    }
  }
  ans
}

fn expand(s: &Stitch, bank: &HashMap<IdOrient, Tile>) -> State {
  let seen: Vec<IdOrient> = s.all_in_path();

  // Finished check
  let possible: HashSet<usize> = bank.keys().map(|(id, _)| *id).collect();
  let seen_id: HashSet<usize> = seen.iter().map(|(id, _)| *id).collect();
  if seen_id.len() == possible.len() {
    return State::Finished;
  }

  // Fit checks
  let mut fits: Vec<Stitch> = Vec::new();
  let unseen: Vec<IdOrient> = bank
    .keys()
    .filter(|(id, _)| !seen_id.contains(id))
    .map(|t| t.to_owned())
    .collect();
  for o in unseen {
    let t = bank.get(&o).unwrap();
    if let Some(me) = &s.l {
      if fits_left(bank.get(&me.id).unwrap(), t) {
        fits.push(Stitch {
          id: o,
          r: Some(me.to_owned()),
          l: None,
          d: None,
          u: None,
        });
      }
    }
    if let Some(me) = &s.r {
      if fits_right(bank.get(&me.id).unwrap(), t) {
        fits.push(Stitch {
          id: o,
          l: Some(me.to_owned()),
          r: None,
          d: None,
          u: None,
        });
      }
    }
    if let Some(me) = &s.u {
      if fits_up(bank.get(&me.id).unwrap(), t) {
        fits.push(Stitch {
          id: o,
          d: Some(me.to_owned()),
          l: None,
          r: None,
          u: None,
        });
      }
    }
    if let Some(me) = &s.d {
      if fits_down(bank.get(&me.id).unwrap(), t) {
        fits.push(Stitch {
          id: o,
          u: Some(me.to_owned()),
          l: None,
          d: None,
          r: None,
        });
      }
    }
  }

  match fits.is_empty() {
    true => State::Dead,
    false => State::Explore(fits),
  }
}

fn fits_left(t1: &Tile, t2: &Tile) -> bool {
  let right = t2
    .img
    .iter()
    .map(|s| s.chars().last().unwrap())
    .collect::<String>();
  let left = t1
    .img
    .iter()
    .map(|s| s.chars().nth(0).unwrap())
    .collect::<String>();
  right == left
}

fn fits_right(t1: &Tile, t2: &Tile) -> bool {
  let right = t1
    .img
    .iter()
    .map(|s| s.chars().last().unwrap())
    .collect::<String>();
  let left = t2
    .img
    .iter()
    .map(|s| s.chars().nth(0).unwrap())
    .collect::<String>();
  right == left
}

fn fits_up(t1: &Tile, t2: &Tile) -> bool {
  t1.img[t1.img.len() - 1] == t2.img[0]
}

fn fits_down(t1: &Tile, t2: &Tile) -> bool {
  t1.img[0] == t2.img[t2.img.len() - 1]
}

fn flip_x(img: &Img) -> Img {
  img.iter().map(|s| s.chars().rev().collect::<String>()).collect()
}

fn rot_r(img: &Img) -> Img {
  let mut new_img: Vec<Vec<char>> = vec![Vec::new(); img.len()];
  img.iter().rev().enumerate().for_each(|(idx,l)| {
    l.chars().for_each(|ch| {
      new_img.get_mut(idx).unwrap().push(ch);
    })
  });
  new_img.iter().map(|chs| chs.iter().collect::<String>()).collect()
}

fn get_all_orientations(img: &Img, id: usize) -> Vec<Tile> {
  let mut res = (0..3)
    .map(|_| rot_r(&img))
    .map(|img| vec![flip_x(&img), img])
    .flatten()
    .enumerate()
    .map(|(oid, img)| Tile {
      img: img.clone(),
      id: (id, (oid as u8) + 1),
    })
    .collect::<Vec<_>>();
  res.push(Tile {
    id: (id, 0),
    img: img.clone(),
  });
  res
}
