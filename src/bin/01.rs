use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left_side, right_side) = parse(input);

    let distance = left_side
        .iter()
        .sorted()
        .zip(right_side.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_side, right_side) = parse(input);

    let mut scores: HashMap<u32, u32> = HashMap::new();
    for a in &left_side {
        for b in &right_side {
            if a == b {
                scores
                    .entry(*a)
                    .and_modify(|score| *score += 1)
                    .or_insert(1);
            }
        }
    }

    let similarities = scores.iter().map(|(item, occurances)| item * occurances);

    Some(similarities.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
