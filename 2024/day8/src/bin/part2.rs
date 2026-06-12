use std::{io, iter};

use rustc_hash::{FxHashMap, FxHashSet};

use day8::Vec2;

fn main() {
    let mut antennae = FxHashMap::default();

    let bounds = day8::find_antennae(io::stdin().lock(), &mut antennae);

    let mut antinodes = FxHashSet::<Vec2>::default();

    for frequency_group in antennae.values() {
        for (a, b) in day8::unique_tuples(&frequency_group) {
            antinodes.extend(find_antinodes(a, b, bounds));
        }
    }

    println!("Number of antinodes: {}", antinodes.len());
}

fn find_antinodes(a: Vec2, b: Vec2, bounds: Vec2) -> impl Iterator<Item = Vec2> {
    let half_iter = |start: Vec2, delta: Vec2, bounds: Vec2| {
        let mut position = start;
        iter::from_fn(move || {
            let new_pos = position + delta;
            new_pos.in_bounds(bounds).then(|| {
                position = new_pos;
                position
            })
        })
    };

    half_iter(a, b - a, bounds).chain(half_iter(b, a - b, bounds))
}
