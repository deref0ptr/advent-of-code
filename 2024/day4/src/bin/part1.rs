use std::{
    fs::File,
    io::{self, BufReader},
};
use itertools::Itertools;
use day4::*;

#[derive(Clone, Copy)]
enum WordDirection {
    XPYN,
    XPY0,
    XPYP,
    X0YP,
    XNYP,
    XNY0,
    XNYN,
    X0YN,
}

impl WordDirection {
    fn apply(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::XPYN | Self::XPY0 | Self::XPYP => x.checked_add(1),
            Self::XNYP | Self::XNY0 | Self::XNYN => x.checked_sub(1),
            _ => Some(x),
        }
        .zip(match self {
            Self::XPYP | Self::X0YP | Self::XNYP => y.checked_add(1),
            Self::XPYN | Self::XNYN | Self::X0YN => y.checked_sub(1),
            _ => Some(y),
        })
    }

    fn iter() -> impl Iterator<Item = Self> + Clone {
        use WordDirection::*;
        static ALL: &[WordDirection; 8] = &[XPYN, XPY0, XPYP, X0YP, XNYP, XNY0, XNYN, X0YN];
        ALL.iter().copied()
    }
}

fn count_occurences(grid: &Grid, text: &[u8]) -> usize {
    let Some((&head, tail)) = text.split_first() else {
        return 0;
    };

    (0..grid.width())
        .cartesian_product(0..grid.height())
        .filter(|&(x, y)| {
            grid.get(x, y)
                .map(Letter::get)
                .is_some_and(|letter| letter == head)
        })
        .cartesian_product(WordDirection::iter())
        .filter(|&((mut x, mut y), direction)| {
            for c in tail {
                let Some(pos) = direction.apply(x, y) else {
                    return false;
                };
                (x, y) = pos;
                if grid.get(x, y).is_none_or(|a| a.get() != *c) {
                    return false;
                }
            }
            true
        })
        .count()
}

fn main() -> io::Result<()> {
    let grid = Grid::parse(BufReader::new(File::open("input.txt")?))?;
    println!("XMAS count: {}", count_occurences(&grid, b"XMAS"));
    Ok(())
}
