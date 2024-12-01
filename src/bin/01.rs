use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (left_side, right_side): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    let distance = std::iter::zip(
        left_side.into_iter().sorted(),
        right_side.into_iter().sorted(),
    )
    .map(|(a, b)| a.abs_diff(b))
    .sum();

    Some(distance)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut left_side = vec![];
    let mut right_side: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left_side.push(items.next().unwrap().parse::<usize>().unwrap());
        right_side
            .entry(items.next().unwrap().parse::<usize>().unwrap())
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    let similarity: usize = left_side
        .iter()
        .map(|number| number * right_side.get(number).unwrap_or(&0))
        .sum();

    Some(similarity)
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
