advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut i = -1;
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|n| n.parse::<u32>().unwrap())
        .partition(|_| {
            i += 1;
            i % 2 == 0
        });
    left.sort_unstable();
    right.sort_unstable();

    let result: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (r.abs_diff(l)))
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
