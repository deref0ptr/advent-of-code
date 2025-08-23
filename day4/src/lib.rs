const LF: u8 = b'\n';
mod letter {
    use core::{fmt, num::NonZero};

    /// Any [u8] value except the linefeed (```b'\n'```)
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Letter(NonZero<u8>);

    impl fmt::Debug for Letter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.get())
        }
    }

    impl fmt::Display for Letter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.get())
        }
    }

    impl Letter {
        const MASK: u8 = super::LF;

        #[inline]
        pub fn new(c: u8) -> Option<Self> {
            NonZero::new(c ^ Self::MASK).map(Self)
        }

        #[inline]
        pub fn get(self) -> u8 {
            self.0.get() ^ Self::MASK
        }
    }
}

use std::{
    io::{self, BufRead},
    ops::Index,
};

pub use self::letter::Letter;

impl TryFrom<u8> for Letter {
    type Error = ();

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl From<Letter> for u8 {
    #[inline]
    fn from(value: Letter) -> Self {
        value.get()
    }
}

#[derive(Debug)]
pub struct Grid {
    repr: Vec<Letter>,
    width: usize,
    height: usize,
}

impl Grid {
    /// Drains the provided row and adds the corresponding letters to the grid
    fn add_row(grid: &mut Vec<Letter>, row: &mut Vec<u8>) {
        grid.extend(row.drain(..).flat_map(Letter::try_from));
    }

    pub fn parse(mut input: impl BufRead) -> io::Result<Self> {
        let mut row_buf = Vec::new();

        let width = input.read_until(LF, &mut row_buf)? - 1;

        let mut repr = Vec::with_capacity(width * width);

        Self::add_row(&mut repr, &mut row_buf);

        let mut height = 1;

        while let Ok(n) = input.read_until(LF, &mut row_buf)
            && n == width + 1
        {
            height += 1;
            Self::add_row(&mut repr, &mut row_buf);
        }

        Ok(Self {
            repr,
            width,
            height,
        })
    }

    #[inline]
    fn convert_index(&self, x: usize, y: usize) -> Option<usize> {
        (x < self.width && y < self.height).then(|| y * self.height + x)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<Letter> {
        self.convert_index(x, y)
            .and_then(|i| self.repr.get(i))
            .copied()
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Letter;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.convert_index(x, y).map(|i| &self.repr[i]).unwrap()
    }
}
