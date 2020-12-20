use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::{HashMap, VecDeque};
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
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      tiles: read_input(d)?
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
          get_all_orientations(&img, id)
        })
        .flatten()
        .collect(),
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
    let upper_l: &Tile = images.get(&traverse(&s, |s| s.u, |s| s.l)).unwrap();
    let upper_r: &Tile = images.get(&traverse(&s, |s| s.u, |s| s.r)).unwrap();
    let lower_l: &Tile = images.get(&traverse(&s, |s| s.d, |s| s.l)).unwrap();
    let lower_r: &Tile = images.get(&traverse(&s, |s| s.d, |s| s.r)).unwrap();
    Ok((upper_l.id.0 * upper_r.id.0 * lower_l.id.0 * lower_r.id.0).to_string())
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
  todo!()
}

fn traverse<F, S>(root: &Stitch, t1: F, t2: S) -> IdOrient
where
  F: Fn(Stitch) -> Option<Box<Stitch>>,
  S: Fn(Stitch) -> Option<Box<Stitch>>,
{
  todo!()
}

fn flip_x(img: &Img) -> Img {
  todo!()
}

fn rot_r(img: &Img) -> Img {
  todo!()
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
