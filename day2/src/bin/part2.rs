use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn check(start: &[u32], end: &[u32]) -> bool {
    let mut iter = start.into_iter().chain(end);

    let Some((&first, &second)) = iter.next().zip(iter.next()) else {
        return true;
    };

    let comparator = match first.cmp(&second) {
        Ordering::Less => PartialOrd::lt,
        Ordering::Greater => PartialOrd::gt,
        Ordering::Equal => return false,
    };

    if first.abs_diff(second) > 3 {
        return false;
    }

    let mut previous = second;

    for &current in iter {
        if !comparator(&previous, &current) || previous.abs_diff(current) > 3 {
            return false;
        }
        previous = current;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = BufReader::new(File::open("input.txt")?);

    let mut line = String::new();
    let mut report = Vec::new();

    let mut safe_reports = 0;

    'outer: while let Ok(1..) = input.read_line(&mut line) {
        report.clear();
        report.extend(line.split_whitespace().flat_map(str::parse::<u32>));
        line.clear();

        if check(&report, &[]) {
            safe_reports += 1;
            continue 'outer;
        } else {
            for i in 0..report.len() {
                let (start, end) = (&report[..i], &report[i + 1..]);
                if check(start, end) {
                    safe_reports += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("Safe reports: {safe_reports}");

    Ok(())
}
