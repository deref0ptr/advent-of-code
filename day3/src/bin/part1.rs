use std::{fs, io};

use regex::Regex;

fn main() -> io::Result<()> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("hardcoded regex");

    let total: u32 = pattern
        .captures_iter(&fs::read_to_string("input.txt")?)
        .map(|c| c.extract::<2>())
        .map(|(_, nums)| {
            nums.iter()
                .copied()
                .flat_map(str::parse::<u32>)
                .product::<u32>()
        })
        .sum();

    println!("Total: {total}");
    Ok(())
}
