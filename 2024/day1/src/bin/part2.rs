use std::{collections::HashMap, error::Error, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut left_list = Vec::with_capacity(1000);
    let mut right_list_counts: HashMap<u32, u32> = HashMap::with_capacity(1000);

    let mut input = BufReader::new(File::open("input.txt")?);
    let mut line = String::with_capacity(16);

    while let Ok(1..) = input.read_line(&mut line) {
        let mut split = line.split_whitespace();

        let mut parse_id = || split.next().and_then(|s| s.parse::<u32>().ok()).ok_or("bad id");
        let id1 = parse_id()?;
        let id2 = parse_id()?;

        left_list.push(id1);
        *right_list_counts.entry(id2).or_default() += 1;

        line.clear();
    }

    let mut similarity_score = 0;

    for id in left_list {
        similarity_score += id * right_list_counts.get(&id).copied().unwrap_or_default();
    }

    println!("Similarity score: {similarity_score}");

    Ok(())
}
