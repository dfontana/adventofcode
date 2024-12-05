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
        Ok(Box::new(find(&self)))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(Box::new(1))
    }
}

fn find(sol: &Solve) -> usize {
    let board = sol.board.chars().collect();
    let dims = (sol.width, sol.height);
    let mut cnt = 0;
    for (pt, _) in sol.board.chars().enumerate().filter(|(_, c)| *c == 'X') {
        cnt += n(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += s(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += e(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += w(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += ne(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += nw(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += se(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
        cnt += sw(dims, &board, pt)
            .filter(|b| *b == ['X', 'M', 'A', 'S'])
            .map(|_| 1)
            .unwrap_or(0);
    }
    cnt
}

// let y = (pt % w) - 1;
// let x = pt - (pt % w);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inp = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....".to_string();
        let solve = Solve::try_from(inp).unwrap();
        assert_eq!(
            solve,
            Solve {
                width: 6,
                height: 5,
                board: "..X....SAMX..A..A.XMAS.S.X....".into(),
            }
        );
        let b = solve.board.chars().collect();
        let dims = (solve.width, solve.height);

        assert_eq!(n(dims, &b, 25), Some(['X', 'M', 'A', 'S']));
        assert_eq!(n(dims, &b, 0), None);
        assert_eq!(e(dims, &b, 0), Some(['.', '.', 'X', '.']));
        assert_eq!(e(dims, &b, 8), Some(['A', 'M', 'X', '.']));
        assert_eq!(e(dims, &b, 9), None);
        assert_eq!(ne(dims, &b, 18), Some(['X', 'A', 'A', '.']));

        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            4
        );
    }
}
