advent_of_code::solution!(1);
use std::iter::zip;

use regex::Regex;

pub fn read_input(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let re = match Regex::new("([0-9]+) +([0-9]+)") {
        Ok(re) => re,
        Err(_) => return None,
    };
    let mut first: Vec<u32> = vec![];
    let mut second: Vec<u32> = vec![];
    for capture in re.captures_iter(input).map(|c| c.extract::<2>().1) {
        first.push(capture[0].parse().unwrap());
        second.push(capture[1].parse().unwrap());
    }
    first.sort();
    second.sort();
    Some((first, second))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (first, second) = match read_input(input) {
        Some(result) => result,
        None => return None,
    };
    Some(
        zip(first, second)
            .map(|(value1, value2)| value1.abs_diff(value2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = match read_input(input) {
        Some(result) => result,
        None => return None,
    };
    let mut sum: u32 = 0;
    for value in first {
        sum += second.iter().filter(|x| value == **x).sum::<u32>()
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(31));
    }
}
