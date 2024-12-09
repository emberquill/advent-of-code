use advent_of_code::Grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let word: Vec<char> = "XMAS".chars().collect();
    let grid = Grid::from_input(input);
    Some(grid.find_word(&word))
}

pub fn part_two(input: &str) -> Option<usize> {
    let word: Vec<char> = "MAS".chars().collect();
    let grid = Grid::from_input(input);
    Some(grid.find_cross_word(&word))
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
