use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_ordering_rules(ordering_rules_input: &str) -> Vec<Vec<bool>> {
    let ordering_rules =
        ordering_rules_input
            .lines()
            .fold(vec![vec![false; 100]; 100], |mut rules, line| {
                let (before, after) = line
                    .split_once('|')
                    .map(|(before, after)| {
                        (
                            before.parse::<usize>().unwrap(),
                            after.parse::<usize>().unwrap(),
                        )
                    })
                    .unwrap();

                rules[before][after] = true;
                rules
            });
    ordering_rules
}

fn parse_pages_line(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|val| val.trim().parse().unwrap())
        .collect_vec()
}

fn adheres_to_rules(values: &[i32], rules: &[Vec<bool>]) -> bool {
    values
        .windows(2)
        .all(|pair| rules[pair[0] as usize][pair[1] as usize])
}

pub fn part_one(input: &str) -> Option<i32> {
    let (ordering_rules_input, values_input) = input.split_once("\n\n").unwrap();

    let ordering_rules = parse_ordering_rules(ordering_rules_input);

    let result = values_input.lines().fold(0, |mut result, line| {
        let page_order = parse_pages_line(line);

        if adheres_to_rules(&page_order, &ordering_rules) {
            result += page_order.get(page_order.len() / 2).unwrap();
        }
        result
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (ordering_rules_input, values_input) = input.split_once("\n\n").unwrap();

    let ordering_rules = parse_ordering_rules(ordering_rules_input);

    let result = values_input.lines().fold(0, |mut result, line| {
        let mut page_order = parse_pages_line(line);

        if adheres_to_rules(&page_order, &ordering_rules) {
            return result;
        }

        page_order.sort_by(|&a, &b| {
            if ordering_rules[a as usize][b as usize] {
                std::cmp::Ordering::Less
            } else if ordering_rules[b as usize][a as usize] {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        result += page_order.get(page_order.len() / 2).unwrap();
        result
    });

    Some(result)
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
