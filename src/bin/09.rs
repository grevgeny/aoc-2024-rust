use std::{convert::Infallible, iter, str::FromStr};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: Disk = input.parse().ok()?;

    let (mut left, mut right) = (0, disk.layout.len() - 1);

    while left < right {
        if disk.layout[left] != -1 {
            left += 1;
            continue;
        }

        if disk.layout[right] == -1 {
            right -= 1;
            continue;
        }

        disk.layout.swap(left, right);
        left += 1;
        right -= 1;
    }

    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Disk = input.parse().ok()?;

    for (file_start, file_end) in disk.file_ranges.clone().into_iter().rev() {
        let file_len = file_end - file_start;

        for free_range in &mut disk.free_ranges {
            let (free_start, free_end) = *free_range;

            if free_start >= file_end {
                break;
            }

            if free_end - free_start >= file_len {
                (free_start..free_end)
                    .take(file_len)
                    .zip(file_start..file_end)
                    .for_each(|(free_idx, file_idx)| {
                        disk.layout.swap(free_idx, file_idx);
                    });

                *free_range = (free_start + file_len, free_end);
                break;
            }
        }
    }

    Some(disk.checksum())
}

#[derive(Debug, Clone)]
struct Disk {
    layout: Vec<i32>,
    free_ranges: Vec<(usize, usize)>,
    file_ranges: Vec<(usize, usize)>,
}

impl Disk {
    fn checksum(&self) -> u64 {
        self.layout
            .iter()
            .enumerate()
            .filter_map(|(i, &el)| {
                if el == -1 {
                    return None;
                }
                Some(i as u64 * el as u64)
            })
            .sum()
    }
}

impl FromStr for Disk {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut layout = Vec::new();
        let mut free_ranges = Vec::new();
        let mut file_ranges = Vec::new();

        let mut disk_id = 0;

        for (i, char) in s.chars().enumerate() {
            let digit = match char.to_digit(10) {
                Some(d) => d as usize,
                None => continue,
            };

            let interval = (layout.len(), layout.len() + digit);

            if i % 2 == 0 {
                file_ranges.push(interval);
                layout.extend(iter::repeat(disk_id).take(digit));
                disk_id += 1;
            } else {
                free_ranges.push(interval);
                layout.extend(iter::repeat(-1).take(digit));
            }
        }

        Ok(Self {
            layout,
            free_ranges,
            file_ranges,
        })
    }
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
        assert_eq!(result, Some(2858));
    }
}
