use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(run_blinks(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run_blinks(input, 75))
}

fn run_blinks(input: &str, total_blinks: usize) -> u64 {
    let stones = parse_input(input);
    let mut history = HashMap::new();

    stones
        .into_iter()
        .map(|stone| stone.blink_recursive(total_blinks, &mut history))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Stone {
    number: u64,
    len: usize,
}

impl Stone {
    fn blink(&self) -> Vec<Self> {
        match self {
            Stone { number: 0, len: 1 } => vec![Stone { number: 1, len: 1 }],
            Stone { number, len } if len % 2 == 0 => {
                let divisor = 10_u64.pow((len / 2) as u32);
                let left = number / divisor;
                let right = number % divisor;

                let left_stone = Stone {
                    number: left,
                    len: (left.checked_ilog10().unwrap_or(0) + 1) as usize,
                };
                let right_stone = Stone {
                    number: right,
                    len: (right.checked_ilog10().unwrap_or(0) + 1) as usize,
                };

                vec![left_stone, right_stone]
            }
            Stone { number, len } => {
                let number = number * 2024;
                let len = if number < 10_u64.pow((len + 3) as u32) {
                    len + 3
                } else {
                    len + 4
                };
                vec![Stone { number, len }]
            }
        }
    }

    fn blink_recursive(self, blinks: usize, history: &mut HashMap<(Stone, usize), u64>) -> u64 {
        if let Some(&result) = history.get(&(self, blinks)) {
            return result;
        }

        if blinks == 0 {
            return 1;
        }

        let result = self
            .blink()
            .into_iter()
            .map(|stone| stone.blink_recursive(blinks - 1, history))
            .sum();

        history.insert((self, blinks), result);
        result
    }
}

fn parse_input(input: &str) -> Vec<Stone> {
    let Some(line) = input.lines().next() else {
        return Vec::new();
    };

    line.split_whitespace()
        .map(|n| Stone {
            number: n.parse().unwrap(),
            len: n.len(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
