use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);

    let mut input = BufReader::new(File::open("input.txt")?);
    let mut line = String::with_capacity(16);

    while let Ok(1..) = input.read_line(&mut line) {
        let mut split = line.split_whitespace();
        let id1: u64 = split.next().and_then(|s| s.parse().ok()).ok_or("should have id1")?;
        list1.push(id1);
        let id2: u64 = split.next().and_then(|s| s.parse().ok()).ok_or("should have id2")?;
        list2.push(id2);

        assert!(split.next().is_none());

        line.clear();
    }

    list1.sort();
    list2.sort();

    let mut total_distance = 0;

    for (id1, id2) in list1.into_iter().zip(list2) {
        total_distance += id1.abs_diff(id2);
    }

    println!("Distance: {total_distance}");

    Ok(())
}
