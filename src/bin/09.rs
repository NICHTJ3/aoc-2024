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
    let mut disk_map = Vec::new();
    let mut id = 0;
    input.chars().enumerate().for_each(|(idx, char)| {
        let is_file = idx % 2 == 0;

        let size = if char.is_ascii_digit() {
            char.to_digit(10).unwrap()
        } else {
            return;
        };

        disk_map.push(if is_file {
            (Some(id), size)
        } else {
            (None, size)
        });

        if is_file {
            id += 1;
        }
    });

    // we can't use a left pointer here because even if we find a can't fit x id we might be able
    // to fit the next block so we have to check most things anyway
    let mut right = disk_map.len() - 1;
    while right > 0 {
        let (file_block_to_move_id, file_block_to_move_size) = disk_map[right];

        if file_block_to_move_id.is_none() {
            right -= 1;
            continue;
        }

        // If we can find a block that will fit the items size
        if let Some(spot_to_move_to) = disk_map[0..right]
            .iter()
            .position(|&(id, size)| id.is_none() && file_block_to_move_size <= size)
        {
            let (_, new_spots_size) = disk_map[spot_to_move_to];

            // Make the swap using the original size of the file block
            disk_map[spot_to_move_to] = (file_block_to_move_id, file_block_to_move_size);
            disk_map[right] = (None, file_block_to_move_size);

            // If we have space left over create a new empty block
            if file_block_to_move_size < new_spots_size {
                disk_map.insert(
                    spot_to_move_to + 1,
                    (None, new_spots_size - file_block_to_move_size),
                );
            }
        }
        right -= 1;
    }

    Some(
        disk_map
            .iter()
            .flat_map(|&(id, size)| (0..size).map(move |_| id))
            .enumerate()
            .flat_map(|(i, id)| id.map(|id| i * id))
            .sum(),
    )
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
        assert_eq!(result, Some(2858));
    }
}
