advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        if let Some((goal, seq)) = parse_line(line) {
            if is_valid_seq_1(&seq[1..], seq[0], goal) {
                sum = sum.saturating_add(goal as u64);
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        if let Some((goal, seq)) = parse_line(line) {
            if is_valid_seq_2(&seq[1..], seq[0], goal) {
                sum = sum.saturating_add(goal as u64);
            }
        }
    }

    Some(sum)
}

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
    let (goal_str, seq_str) = line.split_once(':')?;
    let goal = goal_str.trim().parse().ok()?;
    let seq = seq_str
        .split_whitespace()
        .map(|num| num.parse().ok())
        .collect::<Option<Vec<i64>>>()?;
    Some((goal, seq))
}

fn is_valid_seq_1(sequence: &[i64], acc: i64, goal: i64) -> bool {
    match sequence {
        [] => acc == goal,
        [next, rest @ ..] => {
            let add_result = acc + next;
            let mul_result = acc * next;

            (add_result <= goal && is_valid_seq_1(rest, add_result, goal))
                || (mul_result <= goal && is_valid_seq_1(rest, mul_result, goal))
        }
    }
}

fn is_valid_seq_2(sequence: &[i64], acc: i64, goal: i64) -> bool {
    match sequence {
        [] => acc == goal,
        [next, rest @ ..] => {
            let add_result = acc + next;
            let mul_result = acc * next;
            let concat_result = {
                let next_len = next.checked_ilog10().unwrap_or(0) + 1;
                acc * 10_i64.pow(next_len) + next
            };

            (add_result <= goal && is_valid_seq_2(rest, add_result, goal))
                || (mul_result <= goal && is_valid_seq_2(rest, mul_result, goal))
                || (concat_result <= goal && is_valid_seq_2(rest, concat_result, goal))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
