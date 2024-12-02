use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = prepare_data(input);
    left.sort_unstable();
    right.sort_unstable();

    let result: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (r.abs_diff(l)))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = prepare_data(input);

    let right_counts = right.into_iter().fold(HashMap::new(), |mut acc, el| {
        *acc.entry(el).or_insert(0) += 1;
        acc
    });
    let result = left
        .into_iter()
        .map(|el| el * right_counts.get(&el).copied().unwrap_or(0))
        .sum();

    Some(result)
}

fn prepare_data(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
