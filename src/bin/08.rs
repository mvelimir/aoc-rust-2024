use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antenna_map = HashMap::new();
    let mut antinode_set = HashSet::new();

    let _ = input.lines().enumerate().for_each(|(i, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(j, ch)| antenna_map.entry(ch).or_insert(vec![]).push((i, j)));
    });

    let grid_dims = (input.lines().next().unwrap().len(), input.lines().count());

    antenna_map.keys().for_each(|antenna| {
        let antennas = antenna_map.get(antenna).unwrap();

        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (i_1, j_1) = antennas[i];
                let (i_2, j_2) = antennas[j];

                let i_diff = i_1 as isize - i_2 as isize;
                let j_diff = j_1 as isize - j_2 as isize;
                let a_1 = (i_2 as isize - i_diff, j_2 as isize - j_diff);
                let a_2 = (i_1 as isize + i_diff, j_1 as isize + j_diff);

                if is_in_bounds(grid_dims, a_1) {
                    antinode_set.insert(format!("{},{}", a_1.0, a_1.1));
                }

                if is_in_bounds(grid_dims, a_2) {
                    antinode_set.insert(format!("{},{}", a_2.0, a_2.1));
                }
            }
        }
    });

    Some(antinode_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antenna_map = HashMap::new();
    let mut antinode_set = HashSet::new();

    let _ = input.lines().enumerate().for_each(|(i, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(j, ch)| antenna_map.entry(ch).or_insert(vec![]).push((i, j)));
    });

    let grid_dims = (input.lines().next().unwrap().len(), input.lines().count());

    antenna_map.keys().for_each(|antenna| {
        let antennas = antenna_map.get(antenna).unwrap();

        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (i_1, j_1) = antennas[i];
                let (i_2, j_2) = antennas[j];

                let mut i_diff = i_1 as isize - i_2 as isize;
                let mut j_diff = j_1 as isize - j_2 as isize;
                let gcd = gcd(i_diff, j_diff);
                i_diff = i_diff / gcd;
                j_diff = j_diff / gcd;

                let mut a = (i_2 as isize, j_2 as isize);
                loop {
                    antinode_set.insert(format!("{},{}", a.0, a.1));
                    a = (a.0 - i_diff, a.1 - j_diff);

                    if !is_in_bounds(grid_dims, a) {
                        break;
                    }
                }

                a = (i_2 as isize, j_2 as isize);
                loop {
                    a = (a.0 + i_diff, a.1 + j_diff);

                    if !is_in_bounds(grid_dims, a) {
                        break;
                    } else {
                        antinode_set.insert(format!("{},{}", a.0, a.1));
                    }
                }
            }
        }
    });

    Some(antinode_set.len() as u32)
}

fn is_in_bounds(dims: (usize, usize), point: (isize, isize)) -> bool {
    point.0 >= 0 && point.0 < dims.0 as isize && point.1 >= 0 && point.1 < dims.1 as isize
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
