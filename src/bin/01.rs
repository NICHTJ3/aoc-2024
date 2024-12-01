use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Initialize vectors
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    // Process each line
    for line in input.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        if let (Some(&col1), Some(&col2)) = (values.first(), values.get(1)) {
            list1.push(col1.parse::<u32>().unwrap()); // Add first column to list1
            list2.push(col2.parse::<u32>().unwrap()); // Add second column to list2
        }
    }

    list1.sort();
    list2.sort();

    for (item1, item2) in list1.into_iter().zip(list2.into_iter()).clone() {
        let distance = item1.abs_diff(item2);
        distances.push(distance)
    }

    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Initialize vectors
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    let mut scores: HashMap<u32, u32> = HashMap::new();

    // Process each line
    for line in input.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        if let (Some(&col1), Some(&col2)) = (values.first(), values.get(1)) {
            list1.push(col1.parse::<u32>().unwrap()); // Add first column to list1
            list2.push(col2.parse::<u32>().unwrap()); // Add second column to list2
        }
    }

    for item1 in &list1 {
        for item2 in &list2 {
            if item1 == item2 {
                scores
                    .entry(*item1)
                    .and_modify(|item| *item += 1)
                    .or_insert(1);
            }
        }
    }

    let mut values: Vec<u32> = Vec::new();
    for (item, occurances) in &scores {
        values.push(item * occurances);
    }

    Some(values.iter().sum())
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
