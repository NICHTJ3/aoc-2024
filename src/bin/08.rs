advent_of_code::solution!(8);
use advent_of_code::Vec2;
use itertools::Itertools;
use std::{collections::HashMap, iter};

fn parse_antennas(input: &str) -> HashMap<char, Vec<Vec2>> {
    let mut freq_antennas = HashMap::<char, Vec<Vec2>>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_alphanumeric() {
                freq_antennas
                    .entry(c)
                    .or_default()
                    .push(Vec2::new(x as i32, y as i32));
            }
        });
    });
    freq_antennas
}

fn solve<I>(bounds: Vec2, antennas: HashMap<char, Vec<Vec2>>, iterator: I) -> usize
where
    I: Iterator<Item = i32> + Clone,
{
    let is_within_bounds =
        |point: &Vec2| point.x >= 0 && point.x < bounds.x && point.y >= 0 && point.y < bounds.y;

    let find_antinodes = |antenna_a: &Vec2, antenna_b: &Vec2| {
        [(antenna_a, antenna_b), (antenna_b, antenna_a)]
            .into_iter()
            .fold(
                vec![],
                |mut antinodes, (starting_antenna, ending_antenna)| {
                    for i in iterator.clone() {
                        let antinode_x =
                            starting_antenna.x + i * (starting_antenna.x - ending_antenna.x);
                        let antinode_y =
                            starting_antenna.y + i * (starting_antenna.y - ending_antenna.y);

                        if !is_within_bounds(&Vec2::new(antinode_x, antinode_y)) {
                            break;
                        }
                        antinodes.push((antinode_x, antinode_y));
                    }
                    antinodes
                },
            )
    };

    antennas
        .iter()
        // For each antenna frequency
        .flat_map(|(_, points)| {
            // For every tuple combination of antennas within the frequency
            points
                .iter()
                .tuple_combinations()
                // Find the antinodes for the given antennas
                .flat_map(|(x, y)| find_antinodes(x, y))
        })
        .unique()
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let antennas = parse_antennas(input);
    let map_bounds = Vec2::new(
        input.lines().count() as i32,
        input.lines().next().unwrap().len() as i32,
    );

    // iter::once(1i32) is a trick itterate once with the number 1 as it's only value
    Some(solve(map_bounds, antennas, iter::once(1i32)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let antennas = parse_antennas(input);
    let map_bounds = Vec2::new(
        input.lines().count() as i32,
        input.lines().next().unwrap().len() as i32,
    );

    // 0i32.. is a trick to infinite loop to keep code similar between solutions
    Some(solve(map_bounds, antennas, 0i32..))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
