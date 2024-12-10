use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid = input.parse().ok()?;
    let visited_positions = grid.simulate_1();
    Some(visited_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid = input.parse().ok()?;
    Some(grid.simulate_2() as u32)
}

#[derive(Debug, Clone)]
struct Grid {
    current_pos: (usize, usize),
    current_dir: Direction,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn simulate_1(&mut self) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::from([self.current_pos]);

        while let Some(next_pos) = self.next_position() {
            match self.grid[next_pos.0][next_pos.1] {
                Cell::Obstacle => self.turn_right(),
                Cell::Empty => {
                    self.current_pos = next_pos;
                    visited.insert(next_pos);
                }
            }
        }

        visited
    }

    fn simulate_2(&mut self) -> usize {
        let mut count = 0;

        let (start_pos, start_dir) = (self.current_pos, self.current_dir);

        let visited_positions = self.simulate_1();
        self.current_pos = start_pos;
        self.current_dir = start_dir;

        for (i, j) in visited_positions {
            if (i, j) == start_pos {
                continue;
            }

            self.grid[i][j] = Cell::Obstacle;

            self.current_pos = start_pos;
            self.current_dir = start_dir;

            if self.check_for_loop() {
                count += 1;
            }

            self.grid[i][j] = Cell::Empty;
        }

        count
    }

    fn next_position(&self) -> Option<(usize, usize)> {
        let (dx, dy) = self.current_dir.vector();

        let x = self.current_pos.0 as isize + dx;
        let y = self.current_pos.1 as isize + dy;

        if x >= 0 && y >= 0 && x < self.grid.len() as isize && y < self.grid[0].len() as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    fn turn_right(&mut self) {
        self.current_dir = self.current_dir.turn_right();
    }

    fn check_for_loop(&mut self) -> bool {
        let mut visited_states = HashSet::from([(self.current_pos, self.current_dir)]);

        loop {
            if let Some(next_pos) = self.next_position() {
                match self.grid[next_pos.0][next_pos.1] {
                    Cell::Obstacle => {
                        self.turn_right();
                    }
                    Cell::Empty => {
                        self.current_pos = next_pos;
                        let state = (self.current_pos, self.current_dir);
                        if visited_states.contains(&state) {
                            return true;
                        }
                        visited_states.insert(state);
                    }
                }
            } else {
                return false;
            }
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start_pos = None;
        let mut start_dir = None;

        for (i, line) in s.lines().enumerate() {
            let mut row: Vec<Cell> = Vec::new();

            for (j, ch) in line.chars().enumerate() {
                match ch {
                    '.' => row.push(Cell::Empty),
                    '#' => row.push(Cell::Obstacle),
                    '^' | '<' | 'v' | '>' => {
                        start_pos = Some((i, j));
                        start_dir = Some(Direction::from_char(ch)?);
                        row.push(Cell::Empty);
                    }
                    _ => return Err("Invalid input".into()),
                }
            }

            grid.push(row);
        }

        if grid.is_empty() || grid[0].is_empty() {
            return Err("Grid is empty".into());
        }

        Ok(Self {
            current_pos: start_pos.ok_or("Missing start position")?,
            current_dir: start_dir.ok_or("Missing start direction")?,
            grid,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const DIRECTION_VECTORS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn vector(&self) -> (isize, isize) {
        Self::DIRECTION_VECTORS[*self as usize]
    }

    fn from_char(ch: char) -> Result<Self, String> {
        let direction = match ch {
            '^' => Self::Up,
            '<' => Self::Left,
            'v' => Self::Down,
            '>' => Self::Right,
            _ => return Err("Invalid input".into()),
        };
        Ok(direction)
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Obstacle,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
