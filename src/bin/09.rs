advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Option<usize>> {
    let mut id = 0;
    let mut disk_map = vec![];
    input.chars().enumerate().for_each(|(idx, char)| {
        let number = if char.is_ascii_digit() {
            char.to_digit(10).unwrap()
        } else {
            return;
        };
        let is_file = idx % 2 == 0;

        (0..number).for_each(|_| {
            disk_map.push(is_file.then_some(id));
        });

        id = if is_file { id + 1 } else { id }
    });
    disk_map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk_map = parse(input);

    // Shift the file blocks from the end to the first available empty spot until the points
    // overlap
    let (mut left, mut right) = (0, disk_map.len());
    while left < right {
        if disk_map[left].is_some() {
            left += 1;
        } else {
            right -= 1;
            disk_map.swap(left, right);
        }
    }

    Some(
        disk_map
            .into_iter()
            .enumerate()
            .flat_map(|(index, element)| element.map(|value| index * value))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
