use std::{collections::VecDeque, convert::Infallible, ops::Index, str::FromStr};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let Ok(map) = input.parse::<Map>();

    let mut score = 0;
    for root in &map.trailheads {
        let (s, _) = bfs(&map, root);
        score += s;
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Ok(map) = input.parse::<Map>();

    let mut rating = 0;
    for root in &map.trailheads {
        let (_, r) = bfs(&map, root);
        rating += r;
    }

    Some(rating)
}

struct Map {
    grid: Vec<Vec<u32>>,
    trailheads: Vec<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn adjacent(&self, (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        [
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ]
        .into_iter()
        .filter(move |&(ni, nj)| ni < self.rows && nj < self.cols)
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trailheads = Vec::new();

        let grid: Vec<Vec<u32>> = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(j, b)| {
                        let digit = (b - b'0') as u32;
                        if digit == 0 {
                            trailheads.push((i, j));
                        }
                        digit
                    })
                    .collect()
            })
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        Ok(Self {
            grid,
            trailheads,
            rows,
            cols,
        })
    }
}

impl Index<usize> for Map {
    type Output = [u32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

fn bfs(map: &Map, root: &(usize, usize)) -> (u32, u32) {
    let mut score = 0;
    let mut rating = 0;

    let mut q = VecDeque::from([*root]);

    let mut path_counts = vec![vec![0; map.cols]; map.rows];
    path_counts[root.0][root.1] = 1;

    while let Some((i, j)) = q.pop_front() {
        let current_value = map[i][j];
        let current_path_count = path_counts[i][j];

        if current_value == 9 {
            score += 1;
            rating += current_path_count;
            continue;
        }

        for nb in map.adjacent((i, j)) {
            let nb_value = map[nb.0][nb.1];

            if nb_value.checked_sub(current_value).is_some_and(|v| v == 1) {
                path_counts[nb.0][nb.1] += current_path_count;
                if path_counts[nb.0][nb.1] == current_path_count {
                    q.push_back(nb);
                }
            }
        }
    }

    (score, rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
