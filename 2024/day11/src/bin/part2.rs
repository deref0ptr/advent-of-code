//! The nice and straightforward solution doesn't scale at all well.
//! From observation, the line's length grows by a factor of ~1.52 each blink,
//! so eight stones at the start would turn into approximately 8 * pow(1.52, 75) ~= 3e14 stones,
//! taking up 8 times as many bytes. I sadly do not have 2 PB of memory lying around.
//! Contrary to the puzzle's description, though, the stones' order does not matter;
//! simply tracking how many of each number there is will work.

use std::{collections::HashMap, io, str::FromStr};

#[derive(Debug)]
pub struct Stones {
    inner: HashMap<u64, u64>,
}

impl Stones {
    // API changes!
    pub fn blink(&mut self, kv_cache: &mut Vec<(u64, u64)>) {
        kv_cache.clear();
        kv_cache.extend(self.inner.iter().map(|(&k, &v)| (k, v)));

        for &(stone_value, count) in &*kv_cache {
            *self.inner.get_mut(&stone_value).unwrap() -= count;

            if stone_value == 0 {
                *self.inner.entry(1).or_default() += count;
            } else {
                match stone_value.ilog10() {
                    // This check is equally nice to read as the one in part 1.
                    log10 if log10 % 2 != 0 => {
                        let divisor = 10u64.pow(log10.div_ceil(2));

                        let (left, right) = (stone_value / divisor, stone_value % divisor);

                        *self.inner.entry(left).or_default() += count;
                        *self.inner.entry(right).or_default() += count;
                    },
                    _ => {
                        *self.inner.entry(stone_value * 2024).or_default() += count;
                    },
                }
            }
        }

        self.inner.retain(|_, &mut v| v > 0);
    }
}

impl FromIterator<u64> for Stones {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut inner = HashMap::new();
        for stone in iter {
            *inner.entry(stone).or_default() += 1;
        }

        Self { inner }
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut stones = Stones::from_iter(input.split_whitespace().map(u64::from_str).map(Result::unwrap));

    let mut cache = vec![];

    for _ in 0..75 {
        stones.blink(&mut cache);
    }

    println!("Number of stones: {}", stones.inner.values().sum::<u64>());

    Ok(())
}
