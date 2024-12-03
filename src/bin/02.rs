advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| is_safe(line) as u32).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn is_safe(raw_report: &str) -> bool {
    let mut levels = raw_report
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok());

    let Some(first) = levels.next() else {
        return false;
    };

    let mut prev = first;
    let mut increasing = None;

    levels.all(|current| {
        let diff = current - prev;
        prev = current;

        if !matches!(diff.abs(), 1..=3) {
            return false;
        }

        *increasing.get_or_insert(diff > 0) == (diff > 0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
