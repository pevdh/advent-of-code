use eyre::eyre;
use regex::Regex;
/// Parsing utilities
use std::sync::LazyLock;

static NUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("(-?\\d+)").unwrap());

pub fn parse_nums(str: &str) -> crate::Result<Vec<i64>> {
    let nums = NUM_REGEX
        .captures_iter(str)
        .map(|c| c.get(1).unwrap())
        .map(|c| c.as_str().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Ok(nums)
}

pub fn parse_num_pair(str: &str) -> crate::Result<(i64, i64)> {
    let nums = parse_nums(str)?;
    if nums.len() != 2 {
        return Err(eyre!(
            "expected 2 numbers, but found {} numbers",
            nums.len()
        ));
    }

    Ok((nums[0], nums[1]))
}

pub fn parse_num_triple(str: &str) -> crate::Result<(i64, i64, i64)> {
    let nums = parse_nums(str)?;
    if nums.len() != 3 {
        return Err(eyre!(
            "expected 3 numbers, but found {} numbers",
            nums.len()
        ));
    }

    Ok((nums[0], nums[1], nums[2]))
}

#[cfg(test)]
mod tests {
    use super::parse_nums;

    #[test]
    pub fn test_parse_nums() {
        assert_eq!(
            parse_nums("1 2s3\n4#5 -6").unwrap(),
            vec![1, 2, 3, 4, 5, -6],
        );
    }
}
