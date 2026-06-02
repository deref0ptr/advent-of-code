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
            || calibration_possible(target, acc + n, nums.clone())?
            || calibration_possible(target, concat(acc, n), nums.clone())?)
    } else {
        Ok(acc == target)
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn concat_123_456() {
        let result = concat(123, 456);
        assert_eq!(result, 123456);
    }
}
