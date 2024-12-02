use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(2);

fn is_safe(items: &[usize]) -> usize {
    let mut ordering = Ordering::Equal;

    for (a, b) in items.iter().tuple_windows() {
        let current_ordering = a.cmp(b);

        ordering = ordering.then(current_ordering);

        if !(current_ordering == ordering && (1..=3).contains(&a.abs_diff(*b))) {
            return 0;
        }
    }

    1
}

pub fn part_one(input: &str) -> Option<usize> {
    let sequences = input.lines().map(|line| {
        let items: Vec<usize> = line
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        is_safe(&items)
    });

    Some(sequences.sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences = input.lines().map(|line| {
        let items: Vec<usize> = line
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        if is_safe(&items) == 1 {
            return 1;
        }

        if (0..items.len()).any(|i| {
            let mut y = items.clone();
            y.remove(i);
            is_safe(&y) == 1
        }) {
            return 1;
        }
        0
    });

    Some(sequences.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 2.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 4.into());
    }
}
