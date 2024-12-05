advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = parse_report_levels(line);
                is_safe(&line) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = parse_report_levels(line);
                is_safe_2(&line) as u32
            })
            .sum(),
    )
}

fn is_safe<'a>(levels: impl IntoIterator<Item = &'a i32>) -> bool {
    let mut levels = levels.into_iter();

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

fn is_safe_2(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for n in 0..levels.len() {
        let skipped = levels.iter().take(n).chain(levels.iter().skip(n + 1));
        if is_safe(skipped) {
            return true;
        }
    }

    false
}

fn parse_report_levels(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect()
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
        assert_eq!(result, Some(4));
    }
}
