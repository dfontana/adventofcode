use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

type Img = Vec<String>;

#[derive(Clone, Debug)]
struct Tile {
  id: usize,
  img: Img,
}

impl Default for Tile {
  fn default() -> Tile {
    Tile {
      id: 0usize,
      img: Vec::new(),
    }
  }
}

pub struct Solve {
  tiles: Vec<(usize, Img)>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      tiles: read_input(d)?
        .split("\n\n")
        .map(|t| {
          let mut lines = t.lines();
          let id_line = lines.next().unwrap();
          (
            id_line["Tile ".len()..id_line.len() - 1].parse().unwrap(),
            lines.map(str::to_owned).collect(),
          )
        })
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let sides = (self.tiles.len() as f64).sqrt() as usize;
    let mut grid = Vec::new();
    for _ in 0..sides {
      let mut row = Vec::new();
      for _ in 0..sides {
        row.push(Tile::default());
      }
      grid.push(row);
    }
    let tiles = self.tiles.iter().fold(HashMap::new(), |mut acc, (k, v)| {
      acc.insert(*k, v.clone());
      acc
    });
    if on_tile(&mut grid, &tiles, 0, 0, sides) {
      let checksum = grid[0][0].id
        * grid[0][sides - 1].id
        * grid[sides - 1][0].id
        * grid[sides - 1][sides - 1].id;
      return Ok(checksum.to_string());
    }

    Err("No solve".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn on_tile(
  grid: &mut Vec<Vec<Tile>>,
  remaining: &HashMap<usize, Img>,
  row: usize,
  col: usize,
  sides: usize,
) -> bool {
  if remaining.is_empty() {
    return true;
  }

  let mut nr = row;
  let mut nc = col + 1;
  if nc >= sides {
    nc = 0;
    nr += 1;
  }

  for (id, gr) in remaining.iter() {
    grid[row][col] = Tile {
      id: *id,
      img: gr.clone(),
    };

    let mut remaining = remaining.clone();
    remaining.remove(id);

    for _ in 0..4 {
      grid[row][col].img = rot_l(&grid[row][col].img);
      if is_matching(grid, row, col) {
        if on_tile(grid, &remaining, nr, nc, sides) {
          return true;
        }
      }
    }

    grid[row][col].img = flip_x(&grid[row][col].img);

    for _ in 0..4 {
      grid[row][col].img = rot_l(&grid[row][col].img);
      if is_matching(grid, row, col) {
        if on_tile(grid, &remaining, nr, nc, sides) {
          return true;
        }
      }
    }
  }
  false
}

fn is_matching(grid: &Vec<Vec<Tile>>, row: usize, col: usize) -> bool {
  let tile = &grid[row][col].img;

  if row > 0 {
    let top = &grid[row - 1][col].img;
    if top[top.len() - 1] != tile[0] {
      return false;
    }
  }

  if col > 0 {
    let left = &grid[row][col - 1].img;
    for (l, r) in left.iter().zip(tile.iter()) {
      if l[l.len() - 1..l.len()] != r[0..1] {
        return false;
      }
    }
  }

  true
}

fn flip_x(img: &Img) -> Img {
  img
    .iter()
    .map(|s| s.chars().rev().collect::<String>())
    .collect()
}

fn rot_l(img: &Img) -> Img {
  let mut new_img: Vec<Vec<char>> = vec![Vec::new(); img.len()];
  img.iter().for_each(|l| {
    l.chars().rev().enumerate().for_each(|(row, ch)| {
      new_img.get_mut(row).unwrap().push(ch);
    })
  });
  new_img
    .iter()
    .map(|chs| chs.iter().collect::<String>())
    .collect()
}
