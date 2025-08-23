use std::{
    fs::File,
    io::{self, BufReader},
};

use itertools::Itertools;

use day4::Grid;

fn count_occurences(grid: &Grid) -> usize {
    (0..grid.width() - 2)
        .cartesian_product(0..grid.height() - 2)
        .filter(|&(x, y)| {
            grid[(x + 1, y + 1)].get() == b'A' && matches!(
                [(x, y), (x + 2, y), (x + 2, y + 2), (x, y + 2)].map(|pos| grid[pos].get()),
                [b'M', b'M', b'S', b'S']
                    | [b'S', b'M', b'M', b'S']
                    | [b'S', b'S', b'M', b'M']
                    | [b'M', b'S', b'S', b'M']
            )
        })
        .count()
}

fn main() -> io::Result<()> {
    let grid = Grid::parse(BufReader::new(File::open("input.txt")?))?;
    println!("X-MAS count: {}", count_occurences(&grid));
    Ok(())
}
