use itertools::Itertools;
use rust_util::Day;
use std::{cmp::Ordering, collections::VecDeque, error::Error, fmt::Display};

// ID, start(incl), end(excl)
#[derive(Clone, Debug, PartialEq, Eq)]
struct Block(usize, usize, usize);
#[derive(Debug, PartialEq, Eq)]
pub struct Solve {
    free: Vec<Block>,
    files: Vec<Block>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // ID starts at 0
        // Every other index is a file, 0 is a file.
        // Position is an expanded value so need a  running total
        let mut free = Vec::new();
        let mut files = Vec::new();
        let mut position: usize = 0;
        let mut file_id = 0;
        for (idx, c) in value.trim_end().chars().enumerate() {
            let bsize = c.to_digit(10).unwrap() as usize;
            if bsize == 0 {
                continue;
            }
            if idx % 2 == 0 {
                files.push(Block(file_id, position, position + bsize));
                file_id += 1;
            } else {
                free.push(Block(0, position, position + bsize));
            }
            position += bsize;
        }
        Ok(Solve { free, files })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(checksum(&compact_blocks(
            self.files.clone(),
            self.free.clone(),
        ))))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(checksum(&compact_files(
            self.files.clone(),
            self.free.clone(),
        ))))
    }
}

fn compact_blocks(files: Vec<Block>, free: Vec<Block>) -> Vec<Block> {
    let mut files = VecDeque::from_iter(files);
    let mut free = VecDeque::from_iter(free);
    // Pop from back of Files and pull from front of free. Go until files
    // or free is drained.
    let mut ret = Vec::new();
    while let Some(file) = files.pop_back() {
        let fb = match free.pop_front() {
            Some(fb) if fb.1 < file.1 => fb,
            _ => {
                ret.push(file);
                continue;
            }
        };
        // How much space of FB is left if the file moves in?
        let free_len = fb.2 - fb.1;
        let file_len = file.2 - file.1;
        let cmp = free_len.cmp(&file_len);
        match cmp {
            // Equal sized, move the file over
            Ordering::Equal => ret.push(Block(file.0, fb.1, fb.2)),
            Ordering::Greater => {
                // More free space than file, so split the freespace
                let split_pt = fb.1 + file_len;
                ret.push(Block(file.0, fb.1, split_pt));
                free.push_front(Block(fb.0, split_pt, fb.2));
            }
            Ordering::Less => {
                // Not enough free space than file, so split the file END
                ret.push(Block(file.0, fb.1, fb.2));
                files.push_back(Block(file.0, file.1, file.2 - free_len));
            }
        }
    }
    ret
}

fn compact_files(files: Vec<Block>, free: Vec<Block>) -> Vec<Block> {
    let mut files = VecDeque::from_iter(files);
    let mut free = VecDeque::from_iter(free);
    let mut ret = Vec::new();
    while let Some(file) = files.pop_back() {
        // TODO: Optimize this seek & remove; it's expensive
        // Seek for the first fb that fits file, if any exist.
        let file_len = file.2 - file.1;
        let Some((idx, pfb)) = free
            .iter()
            .find_position(|b| (b.2 - b.1) >= file_len && b.1 < file.1)
        else {
            // No free block big enough, skip file move
            ret.push(file);
            continue;
        };
        let fb = pfb.clone();
        free.remove(idx); // This is costly

        // Below this line can be the same as compact_blocks
        // but free.insert needs to be made less expensive
        // since that always removes/inserts against index 0
        let free_len = fb.2 - fb.1;
        let cmp = free_len.cmp(&file_len);
        match cmp {
            Ordering::Equal => ret.push(Block(file.0, fb.1, fb.2)),
            Ordering::Greater => {
                let split_pt = fb.1 + file_len;
                ret.push(Block(file.0, fb.1, split_pt));
                // This is costly
                free.insert(idx, Block(fb.0, split_pt, fb.2));
            }
            Ordering::Less => {
                ret.push(Block(file.0, fb.1, fb.2));
                files.push_back(Block(file.0, file.1, file.2 - free_len));
            }
        }
    }
    ret
}

fn checksum(disk: &Vec<Block>) -> usize {
    disk.iter()
        .flat_map(|b| (b.1..b.2).map(|p| (p, b.0)))
        .map(|(p, b)| p * b)
        .sum::<usize>()
}
