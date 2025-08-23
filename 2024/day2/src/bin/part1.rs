use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{BufRead, BufReader}, mem,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = BufReader::new(File::open("input.txt")?);

    let mut line = String::new();

    let mut safe_reports = 0;

    while let Ok(1..) = {
        line.clear();
        input.read_line(&mut line)
    } {
        let mut report = line.split_whitespace().flat_map(str::parse::<u32>);

        let Some((first, second)) = report.next().zip(report.next()) else {
            continue;
        };

        if first.abs_diff(second) > 3 {
            continue;
        }

        let comparator: fn(u32, u32) -> bool = match first.cmp(&second) {
            Ordering::Less => |a, b| a < b,
            Ordering::Greater => |a, b| a > b,
            Ordering::Equal => continue,
        };

        let mut previous = second;

        if report.all(|level| comparator(previous, level) && level.abs_diff(mem::replace(&mut previous, level)) <= 3) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");

    Ok(())
}
