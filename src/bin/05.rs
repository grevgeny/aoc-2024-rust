use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

type PeekableLineIterator<'a> = std::iter::Peekable<std::str::Lines<'a>>;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().peekable();
    let ordering_rules = parse_rules(&mut lines);

    let mut total = 0_u32;
    for update_str in lines {
        let update: Vec<&str> = update_str.split(',').collect();
        if is_valid_update(&update, &ordering_rules) {
            total += middle_page_number(&update).unwrap_or(0);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().peekable();
    let ordering_rules = parse_rules(&mut lines);

    let mut total = 0_u32;

    for update_str in lines {
        let mut update: Vec<&str> = update_str.split(',').collect();

        if is_valid_update(&update, &ordering_rules) {
            continue;
        }

        let dag = build_update_dag(&update, &ordering_rules);
        let sorted = topological_sort(&dag)?;

        let position: HashMap<&str, usize> = sorted
            .iter()
            .enumerate()
            .map(|(i, &page)| (page, i))
            .collect();

        update.sort_by_key(|p| position[p]);

        total += middle_page_number(&update).unwrap_or(0);
    }

    Some(total)
}

fn parse_rules<'a>(lines: &mut PeekableLineIterator<'a>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut ordering_rules: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();

    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        if let Some((left, right)) = line.split_once('|') {
            ordering_rules.entry(left).or_default().insert(right);
        }
    }

    assert!(matches!(lines.next(), Some("")));

    ordering_rules
}

fn is_valid_update(update: &[&str], ordering_rules: &HashMap<&str, HashSet<&str>>) -> bool {
    let index_map: HashMap<&str, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for (x, ys) in ordering_rules {
        for &y in ys {
            if let (Some(&x_idx), Some(&y_idx)) = (index_map.get(x), index_map.get(y)) {
                if x_idx > y_idx {
                    return false;
                }
            }
        }
    }

    true
}

struct Dag<'a> {
    adj: HashMap<&'a str, Vec<&'a str>>,
    in_degree: HashMap<&'a str, usize>,
}

impl<'a> Dag<'a> {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
            in_degree: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: &'a str, v: &'a str) {
        self.adj.entry(u).or_default().push(v);
        *self.in_degree.entry(v).or_insert(0) += 1;
        self.in_degree.entry(u).or_insert(0);
    }
}

fn build_update_dag<'a>(
    update: &[&'a str],
    ordering_rules: &HashMap<&'a str, HashSet<&'a str>>,
) -> Dag<'a> {
    let update_set: HashSet<&str> = update.iter().copied().collect();
    let mut dag = Dag::new();

    for &p in update {
        dag.in_degree.entry(p).or_insert(0);
        dag.adj.entry(p).or_default();
    }

    for (x, ys) in ordering_rules {
        if update_set.contains(x) {
            for &y in ys {
                if update_set.contains(y) {
                    dag.add_edge(x, y);
                }
            }
        }
    }

    for &p in &update_set {
        dag.in_degree.entry(p).or_insert(0);
        dag.adj.entry(p).or_default();
    }

    dag
}

fn topological_sort<'a>(dag: &Dag<'a>) -> Option<Vec<&'a str>> {
    let mut in_degree = dag.in_degree.clone();
    let mut queue = VecDeque::new();

    for (&page, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(page);
        }
    }

    let mut result = Vec::with_capacity(in_degree.len());

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(adj_list) = dag.adj.get(node) {
            for &v in adj_list {
                let d = in_degree.get_mut(v).unwrap();
                *d -= 1;
                if *d == 0 {
                    queue.push_back(v);
                }
            }
        }
    }

    if result.len() == in_degree.len() {
        Some(result)
    } else {
        None
    }
}

fn middle_page_number(update: &[&str]) -> Option<u32> {
    update[update.len() / 2].parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
