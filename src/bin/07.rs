use itertools::Itertools;

advent_of_code::solution!(7);

fn is_solvable(remainder: i64, terms: &[i64], allow_concat: bool) -> bool {
    let Some((current_term, remaining_terms)) = terms.split_last() else {
        return remainder == 0;
    };

    // Try adding
    is_solvable(remainder - current_term, remaining_terms, allow_concat)
    // Try multiplying
    || remainder % current_term == 0 && is_solvable(remainder / current_term, remaining_terms, allow_concat)
    // Try concattinating
    || allow_concat &&{
        let remainder = remainder - current_term;
        let current_terms_next_power_of_ten = 10i64.pow(current_term.ilog10() + 1); // e.g. 6 = 10, 15 = 100
        remainder % current_terms_next_power_of_ten == 0 && is_solvable( remainder / current_terms_next_power_of_ten , remaining_terms, allow_concat)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .filter_map(|(target_string, terms_string)| {
                let target = target_string.parse().unwrap();
                let terms = terms_string
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect_vec();

                is_solvable(target, &terms, false).then_some(target)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .filter_map(|(target_string, terms_string)| {
                let target = target_string.parse().unwrap();
                let terms = terms_string
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect_vec();

                is_solvable(target, &terms, true).then_some(target)
            })
            .sum(),
    )
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
        assert_eq!(result, Some(11387));
    }
}
