advent_of_code::solution!(2);

fn test_safety(input: &[i32]) -> bool {
    let windows: Vec<i32> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    windows.iter().all(|window| (1..=3).contains(window))
        || windows.iter().all(|window| (-3..=-1).contains(window))
}

fn read_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        read_lines(input)
            .iter()
            .filter(|&line| test_safety(line))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: usize = 0;
    for line in read_lines(input) {
        if test_safety(&line) {
            result += 1;
            continue;
        }
        for index in 0..line.len() {
            let mut dampened = line.to_vec();
            dampened.remove(index);
            if test_safety(&dampened) {
                result += 1;
                break;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
