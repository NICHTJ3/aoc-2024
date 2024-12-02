advent_of_code::solution!(2);

fn sequence_is_valid(items: Vec<i32>) -> bool {
    if items.len() < 2 {
        return true;
    }

    let mut stack = vec![items[0]];
    let mut direction: Option<bool> = None;

    for &num in &items[1..] {
        let top = stack.last().unwrap();

        let gap = (num - top).abs();

        if !(1..=3).contains(&gap) {
            return false;
        }

        if direction.is_none() {
            direction = Some(num > *top);
        } else if (direction == Some(true) && num <= *top)
            || (direction == Some(false) && num >= *top)
        {
            return false;
        }

        stack.push(num);
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let sequences = input.lines().map(|line| {
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>())
            .map(Result::unwrap)
            .collect();

        sequence_is_valid(items)
    });

    Some(sequences.filter(|item| *item).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let sequences = input.lines().map(|line| {
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>())
            .map(Result::unwrap)
            .collect();

        if sequence_is_valid(items.clone()) {
            return true;
        }

        for i in 0..items.len() {
            let mut reduced_sequence = items.to_vec();
            reduced_sequence.remove(i); // Remove one element.
            if sequence_is_valid(reduced_sequence) {
                return true;
            }
        }
        false
    });

    Some(sequences.filter(|item| *item).count())
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
