advent_of_code::solution!(4);

type Point<T = i32> = (T, T);

const NORTH: Point = (-1, 0);
const SOUTH: Point = (1, 0);
const EAST: Point = (0, 1);
const WEST: Point = (0, -1);
const NORTH_EAST: Point = (-1, 1);
const SOUTH_EAST: Point = (1, 1);
const NORTH_WEST: Point = (-1, -1);
const SOUTH_WEST: Point = (1, -1);

const DIRECTIONS: [Point; 8] = [
    NORTH, SOUTH, EAST, WEST, NORTH_EAST, SOUTH_EAST, NORTH_WEST, SOUTH_WEST,
];

fn out_of_bounds<T>(y: usize, x: usize, matrix: &[Vec<T>]) -> bool {
    y == 0 || x == 0 || y > matrix.len() || x > matrix[0].len()
}

fn search(
    (x, y): Point<usize>,
    matrix: &[Vec<u8>],
    search_term: &[u8],
    search_dir: Point,
    depth: usize,
) -> bool {
    // Base case: If all characters in the search term are matched
    if depth == search_term.len() {
        return true;
    }

    // Boundary checks and value match check
    if out_of_bounds(y, x, matrix) || matrix[y - 1][x - 1] != search_term[depth] {
        return false;
    }

    // Recursive call to move in the specified direction
    let next_x = (x as i32 + search_dir.0) as usize;
    let next_y = (y as i32 + search_dir.1) as usize;

    search((next_x, next_y), matrix, search_term, search_dir, depth + 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let word_to_find = "XMAS".as_bytes();
    let matrix: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    Some(
        (1..=rows)
            .flat_map(|y| (1..=cols).map(move |x| (x, y)))
            .fold(0, |accum, (x, y)| {
                if matrix[y - 1][x - 1] != b'X' {
                    return accum;
                }

                accum
                    + DIRECTIONS
                        .iter()
                        .filter(|dir| search((x, y), &matrix, word_to_find, **dir, 0))
                        .count()
            }),
    )
}

fn apply_direction(point: Point<usize>, direction: Point) -> Point<usize> {
    let next_x = (point.0 as i32 + direction.0) as usize;
    let next_y = (point.1 as i32 + direction.1) as usize;

    (next_x, next_y)
}

// M S S M S M M M S S
//  A   A   A   A   A
// M S S M S M S S M M
pub fn part_two(input: &str) -> Option<u32> {
    let word_to_find = "SAM".as_bytes();
    let matrix: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut occurances = 0;

    for x in 1..cols {
        for y in 1..rows {
            if matrix[y - 1][x - 1] != b'A' {
                continue;
            }

            if (search(
                apply_direction((x, y), NORTH_EAST),
                &matrix,
                word_to_find,
                SOUTH_WEST,
                0,
            ) || search(
                apply_direction((x, y), SOUTH_WEST),
                &matrix,
                word_to_find,
                NORTH_EAST,
                0,
            )) && (search(
                apply_direction((x, y), NORTH_WEST),
                &matrix,
                word_to_find,
                SOUTH_EAST,
                0,
            ) || search(
                apply_direction((x, y), SOUTH_EAST),
                &matrix,
                word_to_find,
                NORTH_WEST,
                0,
            )) {
                occurances += 1;
            }
        }
    }

    Some(occurances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
