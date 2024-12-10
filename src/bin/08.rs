use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::{Add, Sub},
    str::FromStr,
};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let Ok(map) = input.parse::<Map>();
    Some(compute_antinodes(&map, AntinodeMode::DistanceBased).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Ok(map) = input.parse::<Map>();
    Some(compute_antinodes(&map, AntinodeMode::Collinear).len() as u32)
}

#[derive(Default)]
struct Map {
    rows: isize,
    cols: isize,
    antennas: HashMap<char, Vec<Position>>,
}

impl Map {
    fn is_within_bounds(&self, pos: Position) -> bool {
        (0..=self.rows).contains(&pos.0) && (0..=self.cols).contains(&pos.1)
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map::default();

        for (i, line) in s.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                map.rows = map.rows.max(i as isize);
                map.cols = map.cols.max(j as isize);

                if char == '.' {
                    continue;
                }

                map.antennas
                    .entry(char)
                    .or_default()
                    .push(Position(i as isize, j as isize));
            }
        }

        Ok(map)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

enum AntinodeMode {
    DistanceBased,
    Collinear,
}

fn compute_antinodes(map: &Map, mode: AntinodeMode) -> HashSet<Position> {
    let mut antinode_positions = HashSet::new();
    for antenna_positions in map.antennas.values() {
        let candidates = match mode {
            AntinodeMode::DistanceBased => antinode_candidates(antenna_positions, |a, b| {
                distance_based_antinodes(a, b, map)
            }),
            AntinodeMode::Collinear => {
                antinode_candidates(antenna_positions, |a, b| collinear_antinodes(a, b, map))
            }
        };
        for candidate in candidates {
            if map.is_within_bounds(candidate) {
                antinode_positions.insert(candidate);
            }
        }
    }
    antinode_positions
}

fn antinode_candidates<F>(antenna_positions: &[Position], strategy: F) -> Vec<Position>
where
    F: Fn(Position, Position) -> Vec<Position>,
{
    let mut candidates = Vec::new();
    for (i, &pos_a) in antenna_positions.iter().enumerate() {
        for &pos_b in antenna_positions.iter().skip(i + 1) {
            candidates.extend(strategy(pos_a, pos_b));
        }
    }
    candidates
}

fn distance_based_antinodes(p1: Position, p2: Position, map: &Map) -> Vec<Position> {
    vec![p1 + p1 - p2, p2 + p2 - p1]
        .into_iter()
        .filter(|&pos| map.is_within_bounds(pos))
        .collect()
}

fn collinear_antinodes(p1: Position, p2: Position, map: &Map) -> Vec<Position> {
    let delta_x = p2.0 - p1.0;
    let delta_y = p2.1 - p1.1;

    let gcd = gcd(delta_x.abs(), delta_y.abs());
    let step_x = delta_x / gcd;
    let step_y = delta_y / gcd;

    let mut positions = Vec::new();

    positions.extend(
        (1..)
            .map(|k| Position(p1.0 - k * step_x, p1.1 - k * step_y))
            .take_while(|&pos| map.is_within_bounds(pos)),
    );

    positions.extend(
        (0..=gcd)
            .map(|k| Position(p1.0 + k * step_x, p1.1 + k * step_y))
            .filter(|&pos| map.is_within_bounds(pos)),
    );

    positions.extend(
        (gcd + 1..)
            .map(|k| Position(p1.0 + k * step_x, p1.1 + k * step_y))
            .take_while(|&pos| map.is_within_bounds(pos)),
    );

    positions
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
