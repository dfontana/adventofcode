#![allow(warnings)]
use rust_util::Day;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

pub struct Solve {
    tiles: Vec<(usize, usize)>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            tiles: value
                .trim()
                .lines()
                .filter_map(|s| s.split_once(","))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect(),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut max_area = 0;
        for i in 0..self.tiles.len() {
            for j in i..self.tiles.len() {
                let area = area(&self.tiles[i], &self.tiles[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        Ok(Box::new(max_area))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        // Now we have to ensure the rectangles are within the confines of the overall
        // shape the coordinates creates, we can't create rectangles whom leave that space.
        //
        // - Can't just check if all 4 corners are in the shape, the shape could be "U" like.
        // - Could check that all edge tiles are in the shape, brute-forcy.
        //   - "Flood"-fill traverse the overall shape, storing hashset of coordinates in the shape
        //   - For every rectangle considered, create the set of coordinates forming it's edges
        //   - if all edges are subsets of the flood-fill, it's valid
        // - Another option would check just the edges of the shape against each rectangle but
        //   that does need deeper thought handling co-linear edges, etc.
        //
        // Flood filling should be semi trivial so long as you pick a starting point inside the shape and not
        // on an edge or outside it. Since we know all the walls, those can seed our valid_tiles set, and as we
        // visit each neighbor of the seed if it's not in the valid_tiles then we know we can "flood it" (add
        // to valid tiles). We'd continue to explore until no more can be colored.

        // Init the edges
        let mut edges: HashSet<(usize, usize)> = HashSet::new();
        let first = self.tiles.first().unwrap();
        let mut curr = first;
        for n in &self.tiles[1..] {
            let x = n.0;
            let y = n.1;
            match (x == curr.0, y == curr.1) {
                (true, false) => {
                    for i in y.min(curr.1)..=y.max(curr.1) {
                        edges.insert((x, i));
                    }
                }
                (false, true) => {
                    for i in x.min(curr.0)..=x.max(curr.0) {
                        edges.insert((i, y));
                    }
                }
                _ => unreachable!("Input points should always form edges"),
            }
            curr = n;
        }
        match (first.0 == curr.0, first.1 == curr.1) {
            (true, false) => {
                for i in first.1.min(curr.1)..=first.1.max(curr.1) {
                    edges.insert((first.0, i));
                }
            }
            (false, true) => {
                for i in first.0.min(curr.0)..=first.0.max(curr.0) {
                    edges.insert((i, first.1));
                }
            }
            _ => unreachable!("Input points should always form edges"),
        }
        println!("Edges");

        // This flood fill is too much (makes sense) -- 47000 radius circle
        // Maybe try to compress coordinates? Next best thing might be a bitmap
        // Flood fill the area of the rectangle
        let mut fill: HashSet<(usize, usize)> = HashSet::new();
        let seed: (usize, usize) = (14028, 18914);
        let mut to_visit = VecDeque::from_iter(vec![seed]);
        let mut seen = 0;
        while let Some(c) = to_visit.pop_front() {
            if edges.contains(&c) || fill.contains(&c) {
                continue;
            }
            fill.insert(c);
            seen += 1;
            // Push the neighbors on, so long as they are within the coordinate space
            // Diagnals are not needed since we only deal with rectangles, but I guess this
            // flood can fail if two "corners" abut diagonally. Let's see if my input does that
            // I guess...
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = c.0.checked_add_signed(dx);
                let ny = c.1.checked_add_signed(dy);
                if let (Some(nx), Some(ny)) = (nx, ny) {
                    to_visit.push_back((nx, ny));
                }
            }
        }
        println!("Flooded - {seen}");

        // Establish the realm of tiles
        let mut valid_tiles = edges.clone();
        valid_tiles.extend(fill);
        println!("Running checks");

        // Find biggest area made of tiles
        let trem = self.tiles.len() * self.tiles.len() / 2;
        let mut total = trem;
        let mut max_area = 0;
        let mut max_corn = ((0, 0), (0, 0));
        let i = (4155, 36719);
        let j = (94808, 48629);
        println!("{}", valid_tiles.len());
        println!("{}", inside(&i, &j, &valid_tiles));
        // for i in 0..self.tiles.len() {
        //     for j in i..self.tiles.len() {
        //         total -= 1;
        //         if inside(&self.tiles[i], &self.tiles[j], &valid_tiles) {
        //             let area = area(&self.tiles[i], &self.tiles[j]);
        //             if area > max_area {
        //                 max_corn = (self.tiles[i], self.tiles[j]);
        //                 max_area = area;
        //             }
        //         }
        //     }
        //     println!("{total}/{trem}");
        // }
        println!("{max_corn:?}");
        Ok(Box::new(max_area))
    }
}

fn inside(a: &(usize, usize), b: &(usize, usize), valid_tiles: &HashSet<(usize, usize)>) -> bool {
    let (x1, y1) = *a;
    let (x2, y2) = *b;
    if x1 == x2 {
        // Vertical line
        let mut edge = HashSet::new();
        for i in y1.min(y2)..=y1.max(y2) {
            edge.insert((x1, i));
        }
        return edge.is_subset(valid_tiles);
    } else if y1 == y2 {
        // Horizontal line
        let mut edge = HashSet::new();
        for i in x1.min(x2)..=x1.max(x2) {
            edge.insert((i, y1));
        }
        return edge.is_subset(valid_tiles);
    } else {
        // Box -- 2 horizontal, 2 vertical
        let mut edge1 = HashSet::new();
        let mut edge2 = HashSet::new();
        for i in x1.min(x2)..=x1.max(x2) {
            edge1.insert((i, y1));
            edge2.insert((i, y2));
        }
        if !edge1.is_subset(valid_tiles) || !edge2.is_subset(valid_tiles) {
            return false;
        }

        let mut edge1 = HashSet::new();
        let mut edge2 = HashSet::new();
        for i in y1.min(y2)..=y1.max(y2) {
            edge1.insert((x1, i));
            edge2.insert((x2, i));
        }
        edge1.is_subset(valid_tiles) && edge2.is_subset(valid_tiles)
    }
}

fn area((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> usize {
    // +1 b/c the coordinates are "inclusive" so the length is off by 1
    (x2.max(x1) - x1.min(x2) + 1) * (y2.max(y1) - y1.min(y2) + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            50
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            24
        );
    }
}
