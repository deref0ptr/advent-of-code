use std::{
    cmp,
    collections::{HashMap, HashSet},
    env,
    hash::Hash,
    io::{self, BufRead},
    ops::Mul,
};

use ndarray::Array2;
use smallvec::SmallVec;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default, Clone)]
pub struct AdjacencyList<T, N> {
    inner: HashMap<T, N>,
}

impl<T: Hash + Eq, N: Default + Extend<T>> AdjacencyList<T, N>
where
    for<'a> &'a N: IntoIterator<Item = &'a T>,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn vertices(&self) -> <&HashMap<T, N> as IntoIterator>::IntoIter {
        self.inner.iter()
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.inner.entry(from).or_default().extend([to]);
    }
}

impl<T: Copy + Hash + Eq, N: Default + Extend<T>> Mul<Self> for &AdjacencyList<T, N>
where
    for<'a> &'a N: IntoIterator<Item = &'a T>,
{
    type Output = AdjacencyList<T, N>;

    /// This operation is equivalent to matrix multiplication of adjacency matrices.
    fn mul(self, rhs: Self) -> AdjacencyList<T, N> {
        let mut new_adj = AdjacencyList::new();

        for (&vertex, neighbours) in self.inner.iter() {
            for &neighbour in neighbours {
                if let Some(nns) = rhs.inner.get(&neighbour) {
                    for &nn in nns {
                        new_adj.add_edge(vertex, nn);
                    }
                }
            }
        }

        new_adj
    }
}

impl<N: Default + Extend<Vec2>> AdjacencyList<Vec2, N>
where
    for<'a> &'a N: IntoIterator<Item = &'a Vec2>,
{
    /// Construct an adjacency list of positions in the given world map.
    pub fn from_world(world: &Array2<u8>) -> Self {
        let (height, width) = world.dim();

        assert!(cmp::max(height, width) < u16::MAX as usize, "World too big");

        let (height, width) = (height as u16, width as u16);

        let indexer = |y: u16, x: u16| (world[(y as _, x as _)], Vec2 { x, y });

        let mut adj = Self::new();

        for y in 0..height {
            for x in 0..width {
                let (current, current_pos) = indexer(y, x);

                // Don't want to be falling off the edge of the world.
                if x + 1 < width {
                    let (right, right_pos) = indexer(y, x + 1);

                    match right.checked_signed_diff(current) {
                        Some(1) => {
                            adj.add_edge(current_pos, right_pos);
                        },
                        Some(-1) => {
                            adj.add_edge(right_pos, current_pos);
                        },
                        _ => (),
                    }
                }

                // See above.
                if y + 1 < height {
                    let (down, down_pos) = indexer(y + 1, x);

                    match down.checked_signed_diff(current) {
                        Some(1) => {
                            adj.add_edge(current_pos, down_pos);
                        },
                        Some(-1) => {
                            adj.add_edge(down_pos, current_pos);
                        },
                        _ => (),
                    }
                }
            }
        }

        adj
    }
}

pub fn get_map(mut input: impl BufRead) -> anyhow::Result<Array2<u8>> {
    let mut buf = vec![];
    input.read_to_end(&mut buf)?;

    let mut lines = buf.split(|&n| n == b'\n');

    let first_line = lines.next().unwrap().trim_ascii();
    let width = first_line.len();

    let mut map = Array2::zeros((0, width));
    map.push_row(first_line.into())?;

    for line in lines {
        let line = line.trim_ascii();
        if !line.is_empty() {
            map.push_row(line.into())?;
        }
    }

    map -= b'0';

    Ok(map)
}

fn calculate<N>(world: &Array2<u8>) -> usize
where
    N: Default + Extend<Vec2>,
    for<'a> &'a N: IntoIterator<Item = &'a Vec2>,
{
    let l1 = AdjacencyList::<_, N>::from_world(world);

    let l2 = &l1 * &l1;

    let l4 = &l2 * &l2;
    let l8 = &l4 * &l4;

    let length_9_routes = &l8 * &l1;

    // Height is in 0..=9, so a length-9 route must go from 0 to 9 height and is therefore a trail.
    length_9_routes.vertices().flat_map(|(_, edges)| edges).count()
}

#[derive(Debug, Clone, Copy)]
pub enum CountingMethod {
    /// Trails with the same start and end position are considered equal
    StartEnd,
    /// Trails with different positions at any point are considered different.
    UniqueRoute,
}

pub fn run(method: CountingMethod) -> anyhow::Result<()> {
    eprintln!("Remember to pipe in the input file!");

    let world = get_map(io::stdin().lock())?;

    let calculate_fn = match method {
        // Using a hashset ensures that, for a given origin position, another position may appear at most once.
        CountingMethod::StartEnd => calculate::<HashSet<_>>,
        // Using a vector means all possible paths are considered; use SmallVec to reduce heap allocations.
        CountingMethod::UniqueRoute => calculate::<SmallVec<[_; 4]>>,
    };
    let score = calculate_fn(&world);

    println!("{bin} score: {score}", bin = env::args().next().unwrap());

    Ok(())
}
