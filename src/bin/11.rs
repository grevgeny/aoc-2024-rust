advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        let mut stones_new = Vec::new();
        for stone in stones {
            stones_new.extend(stone.blink());
        }
        stones = stones_new;
    }

    Some(stones.len() as u64)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Stone {
    number: u64,
    len: usize,
}

impl Stone {
    fn blink(self) -> Vec<Self> {
        match self {
            Stone { number: 0, len: 1 } => vec![Stone { number: 1, len: 1 }],
            Stone { number, len } if len % 2 == 0 => {
                let number_str = number.to_string();
                let (left, right) = number_str.split_at(len / 2);

                let left_stone = match left.trim_start_matches('0') {
                    "" => Stone { number: 0, len: 1 },
                    l => Stone {
                        number: l.parse().unwrap(),
                        len: l.len(),
                    },
                };
                let right_stone = match right.trim_start_matches('0') {
                    "" => Stone { number: 0, len: 1 },
                    r => Stone {
                        number: r.parse().unwrap(),
                        len: r.len(),
                    },
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
        assert_eq!(result, None);
    }
}
