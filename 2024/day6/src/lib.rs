use std::{
    collections::HashSet,
    error,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    result,
};

#[non_exhaustive]
#[derive(Debug)]
pub enum LabParseError {
    IOError(io::Error),
    BadSymbol(usize, usize),
    NotRectangular,
    MissingGuard,
    TooManyGuards,
}

impl From<io::Error> for LabParseError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for LabParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{err}"),
            Self::BadSymbol(line, col) => write!(f, "Bad character in input file at {line}:{col}"),
            Self::NotRectangular => write!(f, "Provided grid is not rectangular"),
            Self::MissingGuard => write!(f, "No guard found in input"),
            Self::TooManyGuards => write!(f, "Too many guards found in input"),
        }
    }
}

pub type Result<T> = result::Result<T, LabParseError>;

impl error::Error for LabParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2(pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }

    pub fn step(self, old_position: Vec2) -> Option<Vec2> {
        let Vec2(x, y) = old_position;
        match self {
            Self::Up if y > 0 => Some(Vec2(x, y - 1)),
            Self::Down => Some(Vec2(x, y + 1)),
            Self::Left if x > 0 => Some(Vec2(x - 1, y)),
            Self::Right => Some(Vec2(x + 1, y)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Lab {
    pub obstacle_positions: HashSet<Vec2>,
    pub dimensions: Vec2,
}

impl Lab {
    pub fn new(obstacle_positions: HashSet<Vec2>, dimensions: Vec2) -> Self {
        Self {
            obstacle_positions,
            dimensions,
        }
    }

    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.0 < self.dimensions.0 && position.1 < self.dimensions.1
    }
}

/// Parses the specified file as a lab.
///
/// Returns `Ok(lab, guard_position)` on success.
pub fn parse_lab(filename: impl AsRef<Path>) -> Result<(Vec2, Lab)> {
    let mut line_buf = Vec::new();
    let mut input = BufReader::new(File::open(filename)?);

    let mut lab_width = 0;
    let mut lab_height = 0;
    let mut guard_position = None;
    let mut obstacle_positions: HashSet<Vec2> = HashSet::with_capacity(64);

    for y_position in 0usize.. {
        line_buf.clear();
        let 1.. = input.read_until(b'\n', &mut line_buf)? else {
            lab_height = y_position + 1;
            break;
        };

        let line = line_buf.trim_ascii();
        if lab_width == 0 || lab_width == line.len() {
            lab_width = line.len();
        } else {
            return Err(LabParseError::NotRectangular);
        }

        for (x_position, symbol) in line.iter().enumerate() {
            match symbol {
                b'.' => continue,
                b'#' => _ = obstacle_positions.insert(Vec2(x_position, y_position)),
                b'^' => {
                    if guard_position.replace(Vec2(x_position, y_position)).is_some() {
                        return Err(LabParseError::TooManyGuards);
                    }
                }
                _ => return Err(LabParseError::BadSymbol(y_position + 1, x_position + 1)),
            }
        }
    }

    guard_position
        .ok_or(LabParseError::MissingGuard)
        .map(|guard_position| (guard_position, Lab::new(obstacle_positions, Vec2(lab_width, lab_height))))
}
