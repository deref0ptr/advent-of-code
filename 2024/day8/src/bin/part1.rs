use std::io;

use rustc_hash::{FxHashMap, FxHashSet};

use day8::Vec2;

fn main() {
    let mut antennae = FxHashMap::default();

    let bounds = day8::find_antennae(io::stdin().lock(), &mut antennae);

    let mut antinodes = FxHashSet::<Vec2>::default();

    for frequency_group in antennae.values() {
        for (a, b) in day8::unique_tuples(&frequency_group) {
            let delta = b - a;

            let an1 = a - delta;
            if an1.in_bounds(bounds) {
                antinodes.insert(an1);
            }

            let an2 = b + delta;
            if an2.in_bounds(bounds) {
                antinodes.insert(an2);
            }
        }
    }

    println!("Number of antinodes: {}", antinodes.len());
}
