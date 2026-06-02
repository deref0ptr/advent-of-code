use day7::*;

fn main() -> Result<(), Box<dyn Error>> {
    day7_main(calibration_possible)
}

fn calibration_possible(
    target: u64, acc: u64, mut nums: SplitAsciiWhitespace<'_>,
) -> Result<bool, ParseIntError> {
    if acc > target {
        Ok(false)
    } else if let Some(n) = nums.next() {
        let n: u64 = n.trim_ascii().parse()?;
        Ok(calibration_possible(target, acc * n, nums.clone())?
            || calibration_possible(target, acc + n, nums.clone())?)
    } else {
        Ok(acc == target)
    }
}
