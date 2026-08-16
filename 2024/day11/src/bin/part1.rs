use std::{io, mem, str::FromStr};

#[derive(Debug)]
pub struct Stones {
    inner: Vec<u64>,
}

impl Stones {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Simulate a blink of another line of stones, and store the result in this one.
    pub fn blink(&mut self, previous: &Self) {
        self.inner.clear();

        for &stone_value in &previous.inner {
            if stone_value == 0 {
                self.inner.push(1);
            } else {
                match stone_value.ilog10() {
                    // This check is personally a little easier to read than the negation in '!log10.is_multiple_of(2)'
                    log10 if log10 % 2 != 0 => {
                        let divisor = 10u64.pow(log10.div_ceil(2));

                        let left_and_right = [stone_value / divisor, stone_value % divisor];

                        self.inner.extend(left_and_right);
                    },
                    _ => {
                        self.inner.push(stone_value * 2024);
                    },
                }
            }
        }
    }

    pub fn view(&self) -> &[u64] {
        &self.inner
    }
}

impl<T: Into<Vec<u64>>> From<T> for Stones {
    fn from(value: T) -> Self {
        Self { inner: value.into() }
    }
}

fn do_blinks(stones: &mut Stones, num_blinks: usize) {
    let mut temp_stones = Stones::new();
    for _ in 0..num_blinks {
        mem::swap(&mut *stones, &mut temp_stones);
        stones.blink(&temp_stones);
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut stones = Stones::from(
        input
            .split_whitespace()
            .map(u64::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>(),
    );

    do_blinks(&mut stones, 25);

    println!("Number of stones: {}", stones.view().len());

    Ok(())
}
