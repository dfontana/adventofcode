use std::error::Error;

use rust_util::Day;

#[derive(Debug, Eq, PartialEq)]
pub struct Solve {
    width: usize,
    height: usize,
    board: String,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let width = value.lines().nth(0).unwrap().len();
        let height = value.lines().count();
        let board = value.replace('\n', "");
        Ok(Solve {
            width,
            height,
            board,
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(Box::new(find(
            &self,
            vec![['X', 'M', 'A', 'S']],
            'X',
            vec![n, s, e, w, ne, nw, se, sw],
        )))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(Box::new(find(
            &self,
            vec![
                ['M', 'M', 'A', 'S', 'S'],
                ['S', 'M', 'A', 'S', 'M'],
                ['S', 'S', 'A', 'M', 'M'],
                ['M', 'S', 'A', 'M', 'S'],
            ],
            'A',
            vec![x],
        )))
    }
}

fn find<F, T>(sol: &Solve, xmas: Vec<T>, anchor: char, ops: Vec<F>) -> usize
where
    for<'a> F: Extractor<'a, T>,
    T: PartialEq + Eq,
{
    let board = sol.board.chars().collect();
    let dims = (sol.width, sol.height);
    let mut cnt = 0;
    for (pt, _) in sol.board.chars().enumerate().filter(|(_, c)| *c == anchor) {
        for op in ops.iter() {
            cnt += op(dims, &board, pt)
                .filter(|b| xmas.iter().any(|x| *x == *b))
                .map(|_| 1)
                .unwrap_or(0)
        }
    }
    cnt
}

trait Extractor<'a, T>: Fn((usize, usize), &'a Vec<char>, usize) -> Option<T> {}
impl<'a, T, F> Extractor<'a, T> for F
where
    F: Fn((usize, usize), &'a Vec<char>, usize) -> Option<T>,
    T: PartialEq + Eq,
{
}

fn n<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_sub(w)?)?,
        *b.get(pt.checked_sub(2 * w)?)?,
        *b.get(pt.checked_sub(3 * w)?)?,
    ])
}
fn s<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_add(w)?)?,
        *b.get(pt.checked_add(2 * w)?)?,
        *b.get(pt.checked_add(3 * w)?)?,
    ])
}
fn e<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    if pt % w + 3 >= w {
        return None;
    }
    Some([
        *b.get(pt)?,
        *b.get(pt + 1)?,
        *b.get(pt + 2)?,
        *b.get(pt + 3)?,
    ])
}
fn w<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    (pt % w).checked_sub(3)?;
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_sub(1)?)?,
        *b.get(pt.checked_sub(2)?)?,
        *b.get(pt.checked_sub(3)?)?,
    ])
}
fn ne<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    if pt % w + 3 >= w {
        return None;
    }
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_sub(w)? + 1)?,
        *b.get(pt.checked_sub(2 * w)? + 2)?,
        *b.get(pt.checked_sub(3 * w)? + 3)?,
    ])
}
fn nw<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    (pt % w).checked_sub(3)?;
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_sub(w)?.checked_sub(1)?)?,
        *b.get(pt.checked_sub(2 * w)?.checked_sub(2)?)?,
        *b.get(pt.checked_sub(3 * w)?.checked_sub(3)?)?,
    ])
}
fn se<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    if pt % w + 3 >= w {
        return None;
    }
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_add(w)? + 1)?,
        *b.get(pt.checked_add(2 * w)? + 2)?,
        *b.get(pt.checked_add(3 * w)? + 3)?,
    ])
}
fn sw<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 4]> {
    (pt % w).checked_sub(3)?;
    Some([
        *b.get(pt)?,
        *b.get(pt.checked_add(w)?.checked_sub(1)?)?,
        *b.get(pt.checked_add(2 * w)?.checked_sub(2)?)?,
        *b.get(pt.checked_add(3 * w)?.checked_sub(3)?)?,
    ])
}
fn x<'a>((w, _): (usize, usize), b: &'a Vec<char>, pt: usize) -> Option<[char; 5]> {
    (pt % w).checked_sub(1)?;
    if pt % w + 1 >= w {
        return None;
    }
    Some([
        *b.get(pt.checked_sub(w)?.checked_sub(1)?)?, // NW
        *b.get(pt.checked_sub(w)? + 1)?,             // NE
        *b.get(pt)?,
        *b.get(pt.checked_add(w)?.checked_sub(1)?)?, // SW
        *b.get(pt.checked_add(w)? + 1)?,             // SE
    ])
}
