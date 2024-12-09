advent_of_code::solution!(4);

const TARGET: &[char] = &['X', 'M', 'A', 'S'];
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char != 'X' {
                continue;
            }

            for &(di, dj) in &DIRECTIONS {
                if check_target(&grid, i, j, di, dj, TARGET) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == 'A' && is_valid_x_mas_center(&grid, i, j) {
                count += 1;
            }
        }
    }

    Some(count)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn check_target(
    grid: &[Vec<char>],
    i: usize,
    j: usize,
    di: isize,
    dj: isize,
    target: &[char],
) -> bool {
    for (k, &ch) in target.iter().enumerate() {
        let ni = i as isize + k as isize * di;
        let nj = j as isize + k as isize * dj;

        if !in_bounds(grid, ni, nj) || grid[ni as usize][nj as usize] != ch {
            return false;
        }
    }
    true
}

fn in_bounds(grid: &[Vec<char>], ni: isize, nj: isize) -> bool {
    ni >= 0 && nj >= 0 && ni < grid.len() as isize && nj < grid[0].len() as isize
}

fn is_valid_x_mas_center(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // Ensure it's not near the edges
    if i < 1 || i >= rows - 1 || j < 1 || j >= cols - 1 {
        return false;
    }

    // Check diagonals for `MAS` or `SAM` patterns
    let tl_br = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
    let tr_bl = (grid[i - 1][j + 1], grid[i + 1][j - 1]);

    matches_mas_or_sam(tl_br) && matches_mas_or_sam(tr_bl)
}

/// Checks if a diagonal pair matches `MAS` or `SAM`.
fn matches_mas_or_sam(pair: (char, char)) -> bool {
    matches!(pair, ('M', 'S') | ('S', 'M'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
