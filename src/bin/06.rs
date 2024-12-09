use advent_of_code::out_of_bounds;

advent_of_code::solution!(6);

use itertools::Itertools;

const DIRECITONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn walk(
    map: &[Vec<u8>],
    mut row: usize,
    mut col: usize,
    return_visited_cells: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut visited_cells = vec![vec![[false; 4]; map[0].len()]; map.len()];

    let mut guard_moving_direction = 0;

    loop {
        if visited_cells[row][col][guard_moving_direction] {
            return None;
        }

        visited_cells[row][col][guard_moving_direction] = true;

        let (direction_modifier_row, direction_modifier_col) = DIRECITONS[guard_moving_direction];

        let (next_row, next_col) = (
            row as i32 + direction_modifier_row,
            col as i32 + direction_modifier_col,
        );

        // If we're going to leave the map. Return what cells we've visited if we should
        if out_of_bounds(next_row, next_col, map) {
            if !return_visited_cells {
                return Some(Vec::new());
            }
            let visited = (0..map.len())
                .cartesian_product(0..map[0].len())
                .filter(|&(row, col)| visited_cells[row][col].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }

        // If we're going to run into an object, turn left
        if map[next_row as usize][next_col as usize] == b'#' {
            guard_moving_direction = (guard_moving_direction + 1) % 4;
        // Otherwise update the row and colum to our next position
        } else {
            (row, col) = (next_row as usize, next_col as usize);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (guard_start_row, guard_start_col) = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(row, col)| map[row][col] == b'^')
        .unwrap();

    Some(
        walk(&map, guard_start_row, guard_start_col, true)
            .unwrap()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (guard_start_row, guard_start_col) = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(row, col)| map[row][col] == b'^')
        .unwrap();

    let p1 = walk(&map, guard_start_row, guard_start_col, true).unwrap();

    Some(
        p1.iter()
            .filter(|&&(row, col)| {
                // Place an object in the guards way
                map[row][col] = b'#';
                // Check there was a loop
                let ok = walk(&map, guard_start_row, guard_start_col, false).is_none();
                // Remove the object so it doesn't effect future loops
                map[row][col] = b'.';
                ok
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
