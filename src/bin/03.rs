use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result: i32 = 0;
    for (_, [number1, number2]) in pattern.captures_iter(input).map(|c| c.extract()) {
        result += number1.parse::<i32>().unwrap() * number2.parse::<i32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut enabled = true;
    let pattern = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let split = Regex::new(r"[(),]").unwrap();
    for instruction in pattern.find_iter(input).map(|c| c.as_str()) {
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => if enabled {
                let parts: Vec<&str> = split.split(instruction).collect();
                result += parts[1].parse::<i32>().unwrap() * parts[2].parse::<i32>().unwrap();
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
