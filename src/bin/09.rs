advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Vec::new();
    let mut id = 0;

    for (i, char) in input.chars().enumerate() {
        let Some(digit) = char.to_digit(10) else {
            continue;
        };
        if i % 2 == 0 {
            disk.extend(vec![id; digit as usize]);
            id += 1;
        } else {
            disk.extend(vec![-1; digit as usize]);
        }
    }

    let (mut left, mut right) = (0, disk.len() - 1);

    while left < right {
        if disk[left] != -1 {
            left += 1;
            continue;
        }

        if disk[right] == -1 {
            right -= 1;
            continue;
        }

        disk.swap(left, right);
        left += 1;
        right -= 1;
    }

    let checksum = disk
        .iter()
        .enumerate()
        .filter(|&(_, &el)| el != -1)
        .map(|(i, &el)| i as u64 * el as u64)
        .sum();

    Some(checksum)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
