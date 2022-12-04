use rust_util::{Day};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

type Img = Vec<String>;

#[derive(Clone, Debug)]
struct Tile {
  id: usize,
  img: Img,
}

pub struct Solve {
  grid: Vec<Vec<Tile>>,
  sides: usize,
}

impl Tile {
  fn default() -> Tile {
    Tile {
      id: 0usize,
      img: Vec::new(),
    }
  }

  fn new(id: usize, img: Img) -> Tile {
    Tile { id, img }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let tiles = value
      .split("\n\n")
      .map(|t| {
        let mut lines = t.lines();
        let id_line = lines.next().unwrap();
        Tile::new(
          id_line["Tile ".len()..id_line.len() - 1].parse().unwrap(),
          lines.map(str::to_owned).collect(),
        )
      })
      .collect::<Vec<_>>();

    let sides = (tiles.len() as f64).sqrt() as usize;

    let mut grid = Vec::new();
    for _ in 0..sides {
      let mut row = Vec::new();
      for _ in 0..sides {
        row.push(Tile::default());
      }
      grid.push(row);
    }

    let unseen = tiles.iter().fold(HashMap::new(), |mut acc, tile| {
      acc.insert(tile.id, tile.img.clone());
      acc
    });

    if !insert_tile(&mut grid, &unseen, 0, 0, sides) {
      return Err("Failed to build image".into());
    }

    Ok(Solve { grid, sides })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let checksum = self.grid[0][0].id
      * self.grid[0][self.sides - 1].id
      * self.grid[self.sides - 1][0].id
      * self.grid[self.sides - 1][self.sides - 1].id;
    Ok(Box::new(checksum.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let tile_size = self.grid[0][0].img.len();
    let mut img = Vec::new();
    for r in 0..self.sides {
      for img_r in 1..tile_size - 1 {
        let mut row = Vec::new();
        for c in 0..self.sides {
          row.push(self.grid[r][c].img[img_r][1..tile_size - 1].to_owned());
        }
        img.push(row.concat());
      }
    }

    let monsters = scan_image(&img);
    Ok(Box::new(
      (img
        .iter()
        .map(|v| v.chars().filter(|c| *c == '#').count())
        .sum::<usize>()
        - monsters * 15)
        .to_string(),
    ))
  }
}

fn insert_tile(
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
      if tile_fits(grid, row, col) {
        if insert_tile(grid, &remaining, nr, nc, sides) {
          return true;
        }
      }
    }
    grid[row][col].img = flip_x(&grid[row][col].img);
    for _ in 0..4 {
      grid[row][col].img = rot_l(&grid[row][col].img);
      if tile_fits(grid, row, col) {
        if insert_tile(grid, &remaining, nr, nc, sides) {
          return true;
        }
      }
    }
  }
  false
}

fn tile_fits(grid: &Vec<Vec<Tile>>, row: usize, col: usize) -> bool {
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

const MONSTER_H: usize = 3;
const MONSTER_W: usize = 20;
const MONSTER: [&str; 3] = [
  "                  # ",
  "#    ##    ##    ###",
  " #  #  #  #  #  #   ",
];

fn scan_image(input: &Img) -> usize {
  let mut test = input.clone();
  for _ in 0..4 {
    test = rot_l(&test);
    if let Some(ct) = count_monsters(&test) {
      return ct;
    }
  }
  test = flip_x(&test);
  for _ in 0..4 {
    test = rot_l(&test);
    if let Some(ct) = count_monsters(&test) {
      return ct;
    }
  }
  0
}

fn count_monsters(img: &Img) -> Option<usize> {
  let height = img.len();
  let width = img[0].len();
  let mut count = 0;
  for r in 0..=height - MONSTER_H {
    for c in 0..=width - MONSTER_W {
      let matched = MONSTER.iter().enumerate().all(|(i, mask)| {
        mask
          .chars()
          .zip(img[r + i][c..c + MONSTER_W].chars())
          .filter(|(m, i)| *m == '#' && m != i)
          .count()
          == 0
      });
      if matched {
        count += 1;
      }
    }
  }
  match count {
    0 => None,
    v => Some(v),
  }
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
