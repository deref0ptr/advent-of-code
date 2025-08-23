use std::{fs, io};

use pcre2::bytes::Regex;

fn main() -> io::Result<()> {
    /*
     * \G asserts that we are at the end of the last match (or the start of the string if this is the first match),
     *   to catch e.g. 'do()mul(1,2)mul(3,4)'
     * \K resets the match so the preceding part is not included
     */
    let pattern = Regex::new(r"(?:\G|(?:do\(\)))(?:(?!don't\(\))(?:.|\s))*?\Kmul\((\d{1,3}),(\d{1,3})\)").expect("hardcoded regex");

    let total: u32 = pattern
        .captures_iter(&fs::read("input.txt")?)
        .flatten()
        .map(|hit| {
            [&hit[1], &hit[2]].into_iter()
                .flat_map(str::from_utf8)
                .flat_map(str::parse::<u32>)
                .product::<u32>()
        })
        .sum();

    println!("Total: {total}");
    Ok(())
}
