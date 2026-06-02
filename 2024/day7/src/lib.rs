pub use std::{
    error::Error,
    io::{self, Read as _},
    num::ParseIntError,
    str::SplitAsciiWhitespace,
};

pub fn day7_main(
    verifier: impl Fn(u64, u64, SplitAsciiWhitespace<'_>) -> Result<bool, ParseIntError>,
) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let mut calibration_result = 0u64;

    for line in input.lines().map(str::trim_ascii) {
        let (target_s, rest) = line.split_once(':').ok_or("bad")?;

        let target: u64 = target_s.parse()?;

        let (first_s, rest) = rest.trim_ascii().split_once(char::is_whitespace).ok_or("bad")?;

        if verifier(target, first_s.parse()?, rest.split_ascii_whitespace())? {
            calibration_result += target;
        }
    }

    println!("Calibration result: {calibration_result}");

    Ok(())
}
