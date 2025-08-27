use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Cow<'static, str>> {
    let mut input = BufReader::new(File::open("input.txt").map_err(|e| Cow::Owned(e.to_string()))?);
    let mut line_buf = String::with_capacity(32);

    let order = day5::get_order(&mut input, &mut line_buf)?;

    let mut correct_middle_pages = Vec::new();
    let mut update_buf = Vec::new();

    'outer: loop {
        line_buf.clear();
        update_buf.clear();
        let 1.. = input
            .read_line(&mut line_buf)
            .map_err(|e| Cow::Owned(e.to_string()))?
        else {
            break;
        };

        update_buf.extend(line_buf.trim_ascii().split(',').map(str::parse::<u8>).flatten());

        if order.iter().any(|(a, b)| {
            update_buf
                .iter()
                .position(|p| a == p)
                .zip(update_buf.iter().position(|p| b == p))
                .map_or(false, |(a, b)| b < a)
        }) {
            continue 'outer;
        }

        correct_middle_pages.push(update_buf[update_buf.len() / 2] as u32);
    }

    println!(
        "Sum of middle pages of correct updates: {}",
        correct_middle_pages.iter().sum::<u32>()
    );

    Ok(())
}
