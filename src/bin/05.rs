advent_of_code::solution!(5);

fn adheres_to_rules(values: &[i32], rules: &[Vec<bool>]) -> bool {
    values
        .windows(2)
        .all(|pair| rules[pair[0] as usize][pair[1] as usize])
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut parts = input.split("\n\n");

    let ordering_rules =
        parts
            .next()
            .unwrap()
            .lines()
            .fold(vec![vec![false; 100]; 100], |mut rules, line| {
                let rule_parts: Vec<usize> = line
                    .split('|')
                    .map(|part| part.trim().parse::<usize>().unwrap())
                    .collect();

                rules[rule_parts[0]][rule_parts[1]] = true;
                rules
            });

    let result = parts.next().unwrap().lines().fold(0, |mut result, line| {
        let values: Vec<i32> = line
            .split(',')
            .map(|val| val.trim().parse().unwrap())
            .collect();
        if adheres_to_rules(&values, &ordering_rules) {
            result += values.get(values.len() / 2).unwrap();
        }
        result
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut parts = input.split("\n\n");

    let ordering_rules =
        parts
            .next()
            .unwrap()
            .lines()
            .fold(vec![vec![false; 100]; 100], |mut rules, line| {
                let rule_parts: Vec<usize> = line
                    .split('|')
                    .map(|part| part.trim().parse::<usize>().unwrap())
                    .collect();

                rules[rule_parts[0]][rule_parts[1]] = true;
                rules
            });

    let result = parts.next().unwrap().lines().fold(0, |mut result, line| {
        let mut values: Vec<i32> = line
            .split(',')
            .map(|val| val.trim().parse().unwrap())
            .collect();

        if adheres_to_rules(&values, &ordering_rules) {
            return result;
        }

        values.sort_by(|&a, &b| {
            if ordering_rules[a as usize][b as usize] {
                std::cmp::Ordering::Less
            } else if ordering_rules[b as usize][a as usize] {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        result += values.get(values.len() / 2).unwrap();
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
