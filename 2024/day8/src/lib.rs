use std::{
    io::BufRead,
    iter,
    ops::{Add, Sub},
};

use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2(i32, i32);

impl Vec2 {
    pub fn in_bounds(self, bounds: Self) -> bool {
        (0..bounds.0).contains(&self.0) && (0..bounds.1).contains(&self.1)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn find_antennae(mut input: impl BufRead, antennae: &mut FxHashMap<u8, Vec<Vec2>>) -> Vec2 {
    let mut width = None;
    let mut height = 0;

    let mut line = vec![];

    while let Ok(2..) = {
        line.clear();
        input.read_until(b'\n', &mut line)
    } {
        let line = line.trim_ascii();

        if let Some(width) = width {
            assert_eq!(line.len(), width as _);
        } else {
            width = Some(line.len() as _);
        }

        for (x, antenna) in line.iter().copied().enumerate().filter(|(_, s)| *s != b'.') {
            antennae.entry(antenna).or_default().push(Vec2(x as _, height));
        }
        height += 1;
    }

    Vec2(width.unwrap(), height)
}

pub fn unique_tuples<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> {
    let mut left = 0;
    let mut right = 1;

    iter::from_fn(move || {
        if right >= items.len() {
            if left >= items.len() - 2 {
                None
            } else {
                left += 1;
                right = left + 1;
                let pair = (items[left], items[right]);
                right += 1;
                Some(pair)
            }
        } else {
            let pair = (items[left], items[right]);
            right += 1;
            Some(pair)
        }
    })
}

#[cfg(test)]
mod tests {
    use std::array;

    use super::*;

    #[test]
    fn pairs_one_to_five() {
        let nums: [u32; 5] = array::from_fn(|i| 1 + i as u32);

        assert_eq!(
            unique_tuples(&nums).collect::<Vec<_>>(),
            [
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 4),
                (3, 5),
                (4, 5),
            ]
        );
    }
}
