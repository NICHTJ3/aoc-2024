use itertools::Itertools;

advent_of_code::solution!(7);

fn get_result_from_terms(carry: i64, terms: &[i64]) -> bool {
    if carry < 0 || terms.is_empty() {
        return carry == 0;
    }

    let divisible_by_term = carry % terms[0] == 0;

    (divisible_by_term && get_result_from_terms(carry / terms[0], &terms[1..]))// Test multiplication with remaining terms
        || get_result_from_terms(carry - terms[0], &terms[1..]) // Test addition with remaining terms
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .map(|(a, b)| {
                (
                    a.parse::<i64>().unwrap(),
                    b.split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .rev()
                        .collect_vec(),
                )
            })
            .map(|(target, terms)| (target, get_result_from_terms(target, terms.as_slice())))
            .fold(0, |acc, (target, valid)| {
                if valid {
                    return acc + target;
                }
                acc
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
